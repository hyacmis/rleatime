use std::{cmp::Reverse, collections::BinaryHeap, fs::File, io, io::Write, sync::Arc, sync::Mutex};

use futures::future::try_join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::{header::CONTENT_LENGTH, Client};
use tokio::{task, task::JoinHandle};
use url::Url;

use crate::chunk::TaskChunk;
use crate::helper::{Handle, HeapSortWrapper, SortWrapper};

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

    /// 执行器
    pub async fn execute_tasks(
        &self,
        tasks: Vec<TaskChunk>,
        queue: HeapSortWrapper,
    ) -> Result<(), anyhow::Error> {
        // TUI 的声明与显示配置
        let m = MultiProgress::new();
        let sty = ProgressStyle::default_bar()
            .template("{prefix} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-");

        let mut handles: Vec<JoinHandle<Result<(), anyhow::Error>>> = vec![];

        for task in tasks {
            let url = self.url.clone();
            let queue_clone = queue.clone();

            let pb = m.add(ProgressBar::new(task.end_byte - task.start_byte));
            pb.set_style(sty.clone());
            pb.set_message(format!("chunk id {:02}", task.id));

            let handle = task::spawn(async move {
                let chunk = task.download(url, pb).await?;
                // 使用二叉树结构进行保存
                let wrapper = SortWrapper::new(task.id, chunk);
                queue_clone.lock().unwrap().push(Reverse(wrapper));
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
    async fn write_data_to_file(&self, chunk_data: Vec<u8>) -> Result<(), io::Error> {
        let file_name = self.url.path().split('/').last().unwrap();

        let mut file = File::create(file_name)?;

        file.write_all(&chunk_data)?;

        Ok(())
    }

    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        let content_length = self.get_content_length().await?;
        let tasks = self.create_chunk_tasks(content_length);

        let queue: HeapSortWrapper = Arc::new(Mutex::new(BinaryHeap::new()));

        self.execute_tasks(tasks, queue.clone()).await?;

        let data = queue.handle()?;
        self.write_data_to_file(data).await?;

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
