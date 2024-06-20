use std::{
    fs::File,
    io::Write,
    sync::{mpsc, mpsc::Sender},
};

use crate::Args;
use futures::future::try_join_all;
use indicatif::ProgressBar;
use tokio::{task, task::JoinHandle};

use crate::chunk::TaskChunk;
use crate::helper::get_pb;
use crate::task::Task;

pub struct ChannelManager;

impl ChannelManager {
    async fn execute_tasks(
        &self,
        tasks: Vec<TaskChunk>,
        tx: Sender<(usize, Vec<u8>)>,
    ) -> Result<(), anyhow::Error> {
        let (mp, sty) = get_pb();
        let mut handles: Vec<JoinHandle<Result<(), anyhow::Error>>> = vec![];

        for task_chunk in tasks {
            let tx_clone = tx.clone();

            let pb = mp.add(ProgressBar::new(
                task_chunk.end_byte - task_chunk.start_byte,
            ));
            pb.set_style(sty.clone());
            pb.set_message(format!("chunk id {:02}", task_chunk.id));

            let handle = task::spawn(async move {
                let chunk = task_chunk.download(pb).await?;
                tx_clone.send((task_chunk.id, chunk)).unwrap();
                Ok(())
            });

            handles.push(handle);
        }
        // 等待完成
        try_join_all(handles).await?;

        // 清除所有进度条
        mp.clear().unwrap();
        Ok(())
    }

    // 接收并排序下载的数据块
    fn receive_and_sort_chunks(
        &self,
        rx: mpsc::Receiver<(usize, Vec<u8>)>,
    ) -> Vec<Option<Vec<u8>>> {
        let mut data = Vec::new();
        while let Ok((order_id, chunk)) = rx.recv() {
            while data.len() <= order_id {
                data.push(None);
            }
            data[order_id] = Some(chunk);
        }
        data
    }

    // 检查并处理下载数据完整性
    fn check_data_integrity(
        &self,
        data: Vec<Option<Vec<u8>>>,
    ) -> Result<Vec<Vec<u8>>, anyhow::Error> {
        if data.iter().any(|chunk| chunk.is_none()) {
            Err(anyhow::anyhow!(
                "One or more download tasks failed, data contains None."
            ))
        } else {
            Ok(data.into_iter().map(|chunk| chunk.unwrap()).collect())
        }
    }

    // 将数据写入文件
    fn write_data_to_file(&self, data: &[u8], file_name: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(file_name)?;
        file.write_all(data)?;
        Ok(())
    }

    pub async fn execute(&self, args: Args) -> Result<(), anyhow::Error> {
        let mut task = Task::new(&args).await?;
        let tasks = task.generate_chunk_tasks();
        let (tx, rx) = mpsc::channel();

        self.execute_tasks(tasks, tx).await?;
        let data = self.receive_and_sort_chunks(rx);
        let data_cleaned = self.check_data_integrity(data)?;
        if !data_cleaned.is_empty() {
            let combined_data = data_cleaned.concat();
            self.write_data_to_file(&combined_data, &task.file_name)?;
        }
        Ok(())
    }
}
