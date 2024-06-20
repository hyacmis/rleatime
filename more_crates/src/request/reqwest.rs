#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct GetNoParamsResponse {
    origin: String,
}
async fn get_no_params() -> Result<GetNoParamsResponse, Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<GetNoParamsResponse>()
        .await?;
    Ok(resp)
}

#[derive(Deserialize, Debug)]
struct GetParamsResponse {
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
    origin: String,
    url: String,
}

async fn get_params() -> Result<GetParamsResponse, Box<dyn Error>> {
    let url = "https://httpbin.org/get";
    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("key", "value");

    let resp = client
        .get(url)
        .query(&params)
        .send()
        .await?
        .json::<GetParamsResponse>()
        .await?;

    Ok(resp)
}

#[derive(Deserialize, Debug)]
struct PostParamsResponse {
    args: HashMap<String, String>,
    data: String,
    files: HashMap<String, String>,
    form: HashMap<String, String>,
    headers: HashMap<String, String>,
    json: HashMap<String, String>,
    origin: String,
    url: String,
}

async fn post_params() -> Result<PostParamsResponse, Box<dyn Error>> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = Client::new();
    let resp = client
        .post("https://httpbin.org/post")
        .timeout(Duration::from_secs(10)) // 设置超时时间
        .json(&map)
        .send()
        .await?
        .json::<PostParamsResponse>()
        .await?;
    Ok(resp)
}

async fn head() -> Result<u64, Box<dyn Error>> {
    let url = "https://httpbin.org/response-headers";
    let client = Client::new();
    let resp = client.head(url).send().await?;

    let content_length = resp.headers().get("content-length");

    if content_length.is_none() {
        panic!("未获取到content_length");
    }

    let len = content_length
        .unwrap()
        .to_str()?
        .parse::<u64>()
        .unwrap_or(0);
    Ok(len)
}

async fn headers() -> Result<HeaderMap, Box<dyn Error>> {
    let url = "https://httpbin.org/response-headers";
    let client = Client::new();
    let resp = client.head(url).send().await?;

    let head = resp.headers();

    Ok(head.to_owned())
}

#[cfg(test)]
mod test_reqwest {
    use super::*;

    #[tokio::test]
    async fn test_get_no_params() {
        let resp = get_no_params().await.unwrap();

        assert!(!resp.origin.is_empty());
    }

    #[tokio::test]
    async fn test_get_params() {
        let resp = get_params().await.unwrap();

        assert!(resp.args.contains_key("key"));
        assert_eq!(resp.url, "https://httpbin.org/get?key=value");
    }

    #[tokio::test]
    async fn test_post_params() {
        let resp = post_params().await.unwrap();

        assert!(!resp.args.contains_key("key"));
        assert_eq!(resp.url, "https://httpbin.org/post");
        assert_eq!(resp.json["body"], "json");
    }

    #[tokio::test]
    async fn test_head() {
        let len = head().await.unwrap();
        assert_eq!(len, 68)
    }

    #[tokio::test]
    async fn test_headers() {
        let headers = headers().await.unwrap();

        let length = headers.get("content-type").unwrap().to_str().unwrap();
        assert_eq!(length, "application/json");

        let length = headers.get("content-length").unwrap().to_str().unwrap();
        assert_eq!(length, "68");

        let length = headers.get("connection").unwrap().to_str().unwrap();
        assert_eq!(length, "keep-alive");

        let length = headers
            .get("access-control-allow-origin")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(length, "*");

        let length = headers
            .get("access-control-allow-credentials")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(length, "true");
    }
}
