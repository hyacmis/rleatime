use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

use futures::future::try_join_all;
use indicatif::ProgressBar;
use tokio::{task, task::JoinHandle};

use crate::chunk::TaskChunk;
use crate::helper::{get_pb, Handle, HeapSortWrapper, SortWrapper};
use crate::task::Task;

pub struct BinaryHeapManager;

impl BinaryHeapManager {
    async fn execute_tasks(
        &self,
        tasks: Vec<TaskChunk>,
        queue: HeapSortWrapper,
    ) -> Result<(), anyhow::Error> {
        let (mp, sty) = get_pb();
        let mut handles: Vec<JoinHandle<Result<(), anyhow::Error>>> = vec![];
        for task in tasks {
            let queue_clone = queue.clone();

            let pb = mp.add(ProgressBar::new(task.end_byte - task.start_byte));
            pb.set_style(sty.clone());
            pb.set_message(format!("chunk id {:02}", task.id));

            let handle = task::spawn(async move {
                let chunk = task.download(pb).await?;
                let wrapper = SortWrapper::new(task.id, chunk);

                queue_clone.lock().unwrap().push(Reverse(wrapper));
                Ok(())
            });
            handles.push(handle);
        }
        // 等待所有下载任务完成
        try_join_all(handles).await?;

        // 清除所有进度条
        mp.clear().unwrap();

        Ok(())
    }

    fn write_file(&self, data: &[u8], file_name: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(file_name)?;
        file.write_all(data)?;
        Ok(())
    }

    pub async fn execute(&self, url: &str) -> Result<(), anyhow::Error> {
        let mut task = Task::new(url).await?;
        let tasks = task.generate_chunk_tasks();

        let queue: HeapSortWrapper = Arc::new(Mutex::new(BinaryHeap::new()));

        self.execute_tasks(tasks, queue.clone()).await?;

        let data = queue.handle()?;

        if !data.is_empty() {
            self.write_file(&data, &task.file_name)?;
        }
        Ok(())
    }
}
