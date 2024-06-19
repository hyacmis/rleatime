use std::{
    fs::{remove_file, rename, File, OpenOptions},
    io::{Seek, SeekFrom, Write},
    sync::Arc,
    sync::Mutex,
};

use crate::chunk::TaskChunk;
use futures::future::try_join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::{task, task::JoinHandle};

use crate::task::Task;

#[derive(Clone)]
pub struct DownloadManager;

impl DownloadManager {
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

    /// 将数据写入文件
    async fn write_data_to_file(
        &self,
        chunk_start: u64,
        chunk_data: Vec<u8>,
        file: Arc<Mutex<File>>,
    ) -> Result<(), anyhow::Error> {
        let mut file = file.lock().unwrap();

        file.seek(SeekFrom::Start(chunk_start))?;
        file.write_all(&chunk_data)?;

        Ok(())
    }

    /// 执行器
    pub async fn execute_tasks(
        &self,
        tasks: Vec<TaskChunk>,
        file: File,
    ) -> Result<(), anyhow::Error> {
        // TUI 的声明与显示配置
        let m = MultiProgress::new();
        let sty = ProgressStyle::default_bar()
            .template("{prefix} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-");

        let mut handles: Vec<JoinHandle<Result<(), anyhow::Error>>> = vec![];

        // 使用Arc和Mutex以便在多个线程间共享和同步文件句柄
        let self_ref = Arc::new(self.clone());
        let file = Arc::new(Mutex::new(file));

        for task_chunk in tasks {
            let self_clone = self_ref.clone();
            let file_clone = file.clone();

            let pb = m.add(ProgressBar::new(
                task_chunk.end_byte - task_chunk.start_byte,
            ));
            pb.set_style(sty.clone());
            pb.set_message(format!("chunk id {:02}", task_chunk.id));

            let handle = task::spawn(async move {
                let chunk = task_chunk.download(pb).await?;
                self_clone
                    .write_data_to_file(task_chunk.start_byte, chunk, file_clone)
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

    pub async fn execute(&self, url: &str) -> Result<(), anyhow::Error> {
        let mut task = Task::new(url).await;
        let tasks = task.generate_chunk_tasks();

        let file = self.create_empty_file_with_size(&task.file_name_temp, task.length)?;

        if self.execute_tasks(tasks, file).await.is_err() {
            remove_file(&task.file_name_temp)?
        }

        rename(&task.file_name_temp, task.file_name)?;

        Ok(())
    }
}
