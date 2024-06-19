use crate::chunk::TaskChunk;
use regex::Regex;
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_LENGTH};
use reqwest::Client;
use std::env;
use url::Url;

#[derive(Debug)]
pub struct Task {
    pub url: Url,
    pub length: u64,
    pub file_name_temp: String,
    pub file_name: String,
    pub save_path: String,
}

impl Task {
    pub async fn new(url_str: &str) -> Task {
        let url = Url::parse(url_str).expect("这不是一个有效的url");

        let response = Client::new()
            .head(url.as_str())
            .send()
            .await
            .expect("发送 head 请求失败");

        let headers = response.headers();

        let length = headers
            .get(CONTENT_LENGTH)
            .and_then(|x| x.to_str().ok())
            .and_then(|x| x.parse::<u64>().ok())
            .expect("从 header 中获取文件长度失败");

        let content_disposition = headers
            .get(CONTENT_DISPOSITION)
            .and_then(|x| x.to_str().ok())
            .expect("从 header 中获取文件名称失败");

        // 使用正则进行匹配
        let re = Regex::new(r#"filename=["']?([^;"']+)["']?"#).unwrap();

        let name = re
            .captures(content_disposition)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or(content_disposition.to_string());

        let path = env::current_dir().expect("获取当前路径失败");

        Task {
            url,
            length,
            file_name_temp: format!("{name}.download"),
            file_name: name.to_string(),
            save_path: path.to_string_lossy().to_string(),
        }
    }
}

impl Task {
    /// 创建区块的任务队列
    pub fn generate_chunk_tasks(&mut self) -> Vec<TaskChunk> {
        // 计算每个线程下载的字节数
        let chunk_size = 1024 * 1024 * 2u64;
        let content_length = self.length;

        let mut tasks = Vec::new();
        let mut start_byte = 0;

        // 根据每个块的大小计算需要多少次下载
        let range = ((content_length as f64) / (chunk_size as f64)).ceil() as u64;

        println!(
            "content_length: {content_length}. {:6.3}",
            content_length / 1024 / 1024
        );

        for i in 0..range {
            let end_byte = if i == range - 1 {
                content_length
            } else {
                start_byte + chunk_size - 1
            };

            tasks.push(TaskChunk::new(
                i as usize,
                self.url.clone(),
                start_byte,
                end_byte,
            ));

            start_byte += chunk_size;
        }

        println!("chunk totals: {}", tasks.len());

        tasks
    }
}
