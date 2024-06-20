use std::env;
use std::path::PathBuf;

use crate::Args;
use regex::Regex;
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_LENGTH};
use reqwest::Client;
use url::Url;

use crate::chunk::TaskChunk;

#[derive(Debug)]
pub struct Task {
    pub url: Url,
    pub length: u64,
    pub file_name_temp: String,
    pub file_name: String,
}

impl Task {
    pub async fn new(args: &Args) -> Result<Task, anyhow::Error> {
        let url = Url::parse(&args.url).expect("这不是一个有效的url");
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::default()) // 处理重定向的情况
            .build()?;

        let response = client
            .head(url.as_str())
            .send()
            .await
            .expect("发送 head 请求失败");

        let headers = response.headers();

        let length: u64 = match headers.get(CONTENT_LENGTH) {
            None => {
                panic!("从 header 中获取文件长度失败")
            }
            Some(len) => {
                let len_str = len.to_str()?;
                len_str.parse::<u64>()?
            }
        };

        let name: String = match headers.get(CONTENT_DISPOSITION) {
            None => {
                let file_name = url.path().split('/').last().unwrap_or("unknown");
                file_name.to_string()
            }
            Some(s) => {
                let disposition = s.to_str()?;
                // 使用正则进行匹配
                let re = Regex::new(r#"filename=["']?([^;"']+)["']?"#).unwrap();

                re.captures(disposition)
                    .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
                    .unwrap_or(disposition.to_string())
            }
        };

        let path = match &args.output_path {
            None => env::current_dir().expect("获取当前路径失败"),
            Some(path) => PathBuf::from(path),
        };

        let file_name_temp = path.join(format!("{name}.download"));
        let file_name = path.join(name);

        Ok(Task {
            url,
            length,
            file_name_temp: file_name_temp.to_string_lossy().to_string(),
            file_name: file_name.to_string_lossy().to_string(),
        })
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

        tasks
    }
}
