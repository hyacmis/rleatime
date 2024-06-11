use futures::StreamExt;
use reqwest::Client;
use url::Url;

// 定义下载任务结构体
#[derive(Clone)]
pub struct TaskChunk {
    pub start_byte: u64,
    pub end_byte: u64,
}

impl TaskChunk {
    pub fn new(start_byte: u64, end_byte: u64) -> Self {
        Self {
            start_byte,
            end_byte,
        }
    }
}

impl TaskChunk {
    /// 区块下载函数
    pub async fn download(&self, client: Client, url: Url) -> Result<Vec<u8>, anyhow::Error> {
        // 发送 GET 请求以下载文件的特定部分
        let response = client
            .get(url.as_str())
            .header(
                "Range",
                format!("bytes={}-{}", self.start_byte, self.end_byte),
            )
            .send()
            .await?
            .error_for_status()?;

        // 读取下载的数据
        let mut chunk = vec![];
        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            chunk.extend_from_slice(&item?);
        }
        Ok(chunk)
    }
}
