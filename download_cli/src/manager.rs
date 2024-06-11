use std::{cmp::Reverse, fs::File, io::Write};

use futures::future::join_all;
use reqwest::{header::CONTENT_LENGTH, Client};
use tokio::task;
use url::Url;

use crate::chunk::TaskChunk;
use crate::helper::{Handle, HeapSortWrapper, SortWrapper};

#[derive(Clone)]
pub struct DownloadManager {
    pub url: Url,
    pub client: Client,
    pub sort_queue: HeapSortWrapper,
}

impl DownloadManager {
    pub fn new(url: &str, queue: HeapSortWrapper) -> Self {
        Self {
            url: Url::parse(url).expect("这不是一个有效的url"),
            client: Client::new(),
            sort_queue: queue,
        }
    }
}

impl DownloadManager {
    pub async fn get_content_length(&self) -> Result<u64, anyhow::Error> {
        let response = self.client.head(self.url.as_str()).send().await?;
        response
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|x| x.to_str().ok())
            .and_then(|x| x.parse::<u64>().ok())
            .ok_or_else(|| anyhow::anyhow!("Failed to retrieve content length"))
    }

    /// 创建区块的任务队列
    pub fn create_chunk_tasks(&self, content_length: u64) -> Vec<TaskChunk> {
        // 计算每个线程下载的字节数
        let chunk_size = 1024 * 1024 * 2u64;

        let mut tasks = Vec::new();
        let mut start_byte = 0;

        // 根据每个块的大小计算需要多少次下载
        let range = ((content_length as f64) / (chunk_size as f64)).ceil() as u64;
        for i in 0..=range {
            let end_byte = if i == range {
                content_length
            } else {
                // 0- (0+1024-1)
                start_byte + chunk_size - 1
            };

            tasks.push(TaskChunk::new(start_byte, end_byte));
            start_byte += chunk_size;
        }
        println!("chunk totals: {}", tasks.len());
        tasks
    }

    // 执行器
    pub async fn execute_tasks(&self, tasks: Vec<TaskChunk>) -> Result<(), anyhow::Error> {
        let mut handles = vec![];

        for (order_id, task) in tasks.into_iter().enumerate() {
            let queue_clone = self.sort_queue.clone();
            let client = self.client.clone();
            let url = self.url.clone();
            let handle = task::spawn(async move {
                println!(
                    "Downloading chunk id {}, from {} to {}",
                    order_id, task.start_byte, task.end_byte
                );

                let chunk = task.download(client, url).await.unwrap();

                let wrapper = SortWrapper::new(order_id, chunk);
                queue_clone.lock().unwrap().push(Reverse(wrapper));

                println!("Downloading chunk id {order_id:?}. done");
            });

            handles.push(handle);
        }
        // 等待所有下载任务完成
        join_all(handles).await;
        println!("全部下载完成!!");

        let data = self.sort_queue.handle()?;
        if !data.is_empty() {
            self.write_data_to_file(&data)?;
        } else {
            println!("不存在数据")
        }
        Ok(())
    }

    // 将数据写入文件
    pub fn write_data_to_file(&self, data: &[u8]) -> Result<(), std::io::Error> {
        let file_name = self.url.path().split('/').last().unwrap();
        let mut file = File::create(file_name)?;
        file.write_all(data)?;
        Ok(())
    }
}

#[test]
fn test_url() {
    let url_path =
        "https://github.com/usebruno/bruno/releases/download/v1.18.0/bruno_1.18.0_x64_win.exe";

    let url = Url::parse(url_path).expect("这不是一个有效的url");

    let file_name = url.path().split('/').last().unwrap();
    dbg!(url.path());
    assert_eq!(file_name, "bruno_1.18.0_x64_win.exe")
}
