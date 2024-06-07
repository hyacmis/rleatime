use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::{fs::File, io::Write};

use futures::future::join_all;
use futures::StreamExt;
use reqwest::{header::CONTENT_LENGTH, Client};
use tokio::task;

// 定义下载任务结构体
#[derive(Clone)]
struct DownloadTask {
    url: String,
    start_byte: u64,
    end_byte: u64,
}

// 用于排序的数据块包装器
#[derive(Eq, PartialEq)]
struct OrderWrapper {
    order_id: usize,
    data: Vec<u8>,
}

impl Ord for OrderWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order_id.cmp(&other.order_id)
    }
}

impl PartialOrd for OrderWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// 区块下载函数
async fn download_chunk(client: &Client, task: &DownloadTask) -> Result<Vec<u8>, anyhow::Error> {
    // 发送 GET 请求以下载文件的特定部分
    let response = client
        .get(&task.url)
        .header(
            "Range",
            format!("bytes={}-{}", task.start_byte, task.end_byte),
        )
        .send()
        .await?
        .error_for_status()?;

    // 读取下载的数据
    let mut chunk = vec![];
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        chunk.extend_from_slice(&item?);
    }
    Ok(chunk)
}

fn create_download_tasks(url: &str, content_length: u64) -> Vec<DownloadTask> {
    // 计算每个线程下载的字节数
    let chunk_size = 1024 * 1024 * 2u64;

    let mut tasks = Vec::new();
    let mut start_byte = 0;

    // 根据每个块的大小计算需要多少次下载
    let range = ((content_length as f64) / (chunk_size as f64)).ceil() as u64;
    for _ in 0..range {
        let end_byte = start_byte + chunk_size;
        tasks.push(DownloadTask {
            url: url.to_string(),
            start_byte,
            end_byte,
        });
        // 加1 保证字节不重复
        start_byte = end_byte + 1;
    }

    // 如有必要，调整最后一个任务的结束字节
    if let Some(last_task) = tasks.last_mut() {
        if last_task.end_byte > content_length {
            last_task.end_byte = content_length;
        }
    }

    tasks
}

// 发送任务到通道的辅助函数
async fn execute_download_tasks(client: Client, tasks: Vec<DownloadTask>, queue: HeapOrderWrapper) {
    let mut handles = vec![];
    for (order_id, task) in tasks.into_iter().enumerate() {
        let client_clone = client.clone();
        let queue_clone = queue.clone();
        let handle = task::spawn(async move {
            println!(
                "Downloading chunk id {}, from {} to {}",
                order_id, task.start_byte, task.end_byte
            );
            let chunk = download_chunk(&client_clone, &task).await.unwrap();
            println!("Downloading chunk id {order_id:?} done");
            let wrapper = OrderWrapper {
                order_id,
                data: chunk,
            };
            queue_clone.lock().unwrap().push(Reverse(wrapper));
        });
        handles.push(handle);
    }
    // 等待所有下载任务完成
    join_all(handles).await;
}

// 接收并排序下载的数据块
fn receive_and_sort_chunks(queue: HeapOrderWrapper) -> Vec<u8> {
    // 提取并合并数据块，注意：由于是最大堆，我们实际上是反向遍历
    let mut data = Vec::new();
    while let Some(Reverse(wrapper)) = queue.lock().unwrap().pop() {
        println!("ord {}", wrapper.order_id);
        data.extend_from_slice(&wrapper.data);
    }
    data
}

// 辅助函数提取内容长度
fn extract_content_length(response: &reqwest::Response) -> Result<u64, anyhow::Error> {
    response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|x| x.to_str().ok())
        .and_then(|x| x.parse::<u64>().ok())
        .ok_or_else(|| anyhow::anyhow!("Failed to retrieve content length"))
}

// 辅助函数从URL中提取文件名
fn extract_filename_from_url(url: &str) -> &str {
    url.split('/').last().unwrap()
}

// 将数据写入文件
fn write_data_to_file(data: &[u8], file_name: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(data)?;
    Ok(())
}
type HeapOrderWrapper = Arc<Mutex<BinaryHeap<Reverse<OrderWrapper>>>>;

// 修改主下载函数以使用新函数
pub async fn download(url: &str) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let response = client.head(url).send().await?;
    let content_length = extract_content_length(&response)?;

    let tasks = create_download_tasks(url, content_length);

    let order_queue: HeapOrderWrapper = Arc::new(Mutex::new(BinaryHeap::new()));

    execute_download_tasks(client, tasks, order_queue.clone()).await;

    let data = receive_and_sort_chunks(order_queue);

    if !data.is_empty() {
        let file_name = extract_filename_from_url(url);
        write_data_to_file(&data, file_name)?;
    }

    println!("Download completed successfully!");
    Ok(())
}
