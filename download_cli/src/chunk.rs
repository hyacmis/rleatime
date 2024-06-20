use futures::StreamExt;
use indicatif::ProgressBar;
use reqwest::{header::RANGE, Client};
use url::Url;

// 定义下载任务结构体
#[derive(Debug, Clone)]
pub struct TaskChunk {
    pub id: usize,
    pub url: Url,
    pub start_byte: u64,
    pub end_byte: u64,
}

impl TaskChunk {
    pub fn new(id: usize, url: Url, start_byte: u64, end_byte: u64) -> Self {
        Self {
            id,
            url,
            start_byte,
            end_byte,
        }
    }
}

impl TaskChunk {
    /// 区块下载函数
    pub async fn download(&self, pb: ProgressBar) -> Result<Vec<u8>, anyhow::Error> {
        // 发送 GET 请求以下载文件的特定部分
        let response = Client::new()
            .get(self.url.as_str())
            .header(
                RANGE,
                format!("bytes={}-{}", self.start_byte, self.end_byte),
            )
            .send()
            .await?
            .error_for_status()?;

        // 读取下载的数据
        let mut chunk = vec![];
        let mut stream = response.bytes_stream();

        assert!(self.start_byte <= self.end_byte);

        while let Some(item) = stream.next().await {
            let item = item?;
            chunk.extend_from_slice(&item);
            // pb 在内部好像会自己累加, 所以这里传递的是每次获取的字节长度
            pb.inc(item.len() as u64);
        }

        pb.finish_with_message(format!("chunk id {:02}: 下载完成", self.id));
        Ok(chunk)
    }
}
