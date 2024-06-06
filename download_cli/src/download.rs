use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use futures::future::join_all;
use reqwest::header::CONTENT_DISPOSITION;
use reqwest::Client;
use tokio::sync::Mutex;

// 定义下载任务结构体
struct DownloadTask {
    url: String,
    start_byte: u64,
    end_byte: u64,
}

// 区块下载函数
async fn download_chunk(client: &Client, task: &DownloadTask) -> Result<Vec<u8>, anyhow::Error> {
    println!(
        "Downloading chunk from {} to {}",
        task.start_byte, task.end_byte
    );

    // 发送 GET 请求以下载文件的特定部分
    let response = client
        .get(&task.url)
        .header(
            "Range",
            format!("bytes={}-{}", task.start_byte, task.end_byte),
        )
        .send()
        .await?;

    // 读取下载的数据
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

fn create_download_tasks(url: &str, num_threads: u64, content_length: u64) -> Vec<DownloadTask> {
    // 计算每个线程下载的字节数
    let chunk_size = content_length / num_threads;

    let mut tasks = Vec::new();
    let mut start_byte = 0;

    for _ in 0..content_length {
        let end_byte = start_byte + chunk_size;
        tasks.push(DownloadTask {
            url: url.to_string(),
            start_byte,
            end_byte,
        });
        start_byte = end_byte;
    }
    // 如有必要，调整最后一个任务的结束字节
    if let Some(last_task) = tasks.last_mut() {
        if last_task.end_byte > content_length {
            last_task.end_byte = content_length;
        }
    }

    tasks
}
fn get_filename_from_response(response: &reqwest::Response) -> String {
    if let Some(content_disposition) = response.headers().get(CONTENT_DISPOSITION) {
        if let Ok(content_disposition) = content_disposition.to_str() {
            if let Some(filename) = content_disposition.split("filename=").nth(1) {
                return filename.trim_matches('"').to_string();
            }
        }
    }
    "unknown_filename".to_string()
}

pub async fn download(url: &str) -> Result<(), anyhow::Error> {
    let num_threads = 4;
    // 创建 reqwest HTTP 客户端
    let client = Client::new();

    // 发送 HEAD 请求以获取文件大小
    let response = client.head(url).send().await?;

    let content_length = response.content_length().unwrap_or(0);

    // 构建下载任务
    let tasks = create_download_tasks(url, num_threads, content_length);

    // 使用 Arc 和 Mutex 来在多个线程之间共享状态
    let data = Arc::new(Mutex::new(Vec::new()));

    // 创建并启动多个异步任务
    let mut handles = vec![];
    for task in tasks {
        let client_clone = client.clone();
        let data_clone = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let chunk = download_chunk(&client_clone, &task).await.unwrap();
            let mut data = data_clone.lock().await;
            data.push(chunk);
        });
        handles.push(handle);
    }

    // 等待所有下载任务完成
    join_all(handles).await;
    let data = data.lock().await;
    if !data.is_empty() {
        // 合并所有下载的数据
        let name = get_filename_from_response(&response);
        let mut file = File::create(name)?;
        for chunk in data.iter() {
            file.write_all(chunk)?;
        }
    }

    println!("Download completed successfully!");

    Ok(())
}
