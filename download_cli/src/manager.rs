use std::{
    fs::{File, OpenOptions},
    io,
    io::{Seek, SeekFrom, Write},
    sync::Arc,
    sync::Mutex,
};

use futures::future::try_join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::{header::CONTENT_LENGTH, Client};
use tokio::{task, task::JoinHandle};
use url::Url;

use crate::chunk::TaskChunk;

#[derive(Clone)]
pub struct DownloadManager {
    pub url: Url,
}

impl DownloadManager {
    pub fn new(url: &str) -> Self {
        Self {
            url: Url::parse(url).expect("这不是一个有效的url"),
        }
    }
}

impl DownloadManager {
    pub async fn get_content_length(&self) -> Result<u64, anyhow::Error> {
        let response = Client::new().head(self.url.as_str()).send().await?;
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

        println!(
            "content_length: {content_length}. {:6.3}",
            content_length / 1024 / 1024
        );
        for i in 0..range {
            let end_byte = start_byte + chunk_size - 1;

            tasks.push(TaskChunk::new(i as usize, start_byte, end_byte));
            start_byte += chunk_size;
        }
        // 处理最后一个任务
        if start_byte < content_length {
            let end_byte = content_length - 1;
            tasks.push(TaskChunk::new(range as usize, start_byte, end_byte));
        }

        println!("chunk totals: {}", tasks.len());
        tasks
    }

    fn create_empty_file_with_size(
        &self,
        file_path: &str,
        size_in_bytes: u64,
    ) -> Result<File, anyhow::Error> {
        // 创建文件
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)?;

        // 直接写入零字节到文件以设定大小
        let buffer = vec![0; size_in_bytes as usize];
        file.write_all(&buffer)?;

        Ok(file)
    }
    /// 执行器
    pub async fn execute_tasks(&self, tasks: Vec<TaskChunk>) -> Result<(), anyhow::Error> {
        // TUI 的声明与显示配置
        let m = MultiProgress::new();
        let sty = ProgressStyle::default_bar()
            .template("{prefix} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-");

        let mut handles: Vec<JoinHandle<Result<(), anyhow::Error>>> = vec![];

        let self_ref = Arc::new(self.clone());

        // 使用Arc和Mutex以便在多个线程间共享和同步文件句柄
        let file_name = self.url.path().split('/').last().unwrap();

        let file = self.create_empty_file_with_size(file_name, tasks.last().unwrap().end_byte)?;

        let file = Arc::new(Mutex::new(file));

        for task in tasks {
            let url = self.url.clone();
            let self_clone = self_ref.clone();
            let file_clone = file.clone();

            let pb = m.add(ProgressBar::new(task.end_byte - task.start_byte));
            pb.set_style(sty.clone());
            pb.set_message(format!("chunk id {:02}", task.id));

            let handle = task::spawn(async move {
                let chunk = task.download(url, pb).await?;
                self_clone
                    .write_data_to_file(task.start_byte, chunk, file_clone)
                    .await?;
                Ok(())
            });

            handles.push(handle);
        }

        // 等待完成
        try_join_all(handles).await?;

        // 清除所有进度条
        m.clear().unwrap();

        Ok(())
    }

    /// 将数据写入文件
    async fn write_data_to_file(
        &self,
        chunk_start: u64,
        chunk_data: Vec<u8>,
        file: Arc<Mutex<File>>,
    ) -> Result<(), io::Error> {
        let mut file = file.lock().unwrap();

        file.seek(SeekFrom::Start(chunk_start))?;
        file.write_all(&chunk_data)?;

        Ok(())
    }

    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        let content_length = self.get_content_length().await?;
        let tasks = self.create_chunk_tasks(content_length);

        self.execute_tasks(tasks).await?;

        Ok(())
    }
}

#[test]
fn test_url_path() {
    let url_path =
        "https://github.com/usebruno/bruno/releases/download/v1.18.0/bruno_1.18.0_x64_win.exe";

    let url = Url::parse(url_path).expect("这不是一个有效的url");

    let file_name = url.path().split('/').last().unwrap();
    assert_eq!(file_name, "bruno_1.18.0_x64_win.exe")
}
