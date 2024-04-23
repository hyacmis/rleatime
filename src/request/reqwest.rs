#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

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
    let client = reqwest::Client::new();

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

    let client = reqwest::Client::new();
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

#[cfg(test)]
mod test_reqwest {
    use super::*;

    #[tokio::main]
    #[test]
    async fn test_get_no_params() {
        let resp = get_no_params().await.unwrap();

        assert!(!resp.origin.is_empty());
    }

    #[tokio::main]
    #[test]
    async fn test_get_params() {
        let resp = get_params().await.unwrap();

        assert!(resp.args.get("key").is_some());
        assert_eq!(resp.url, "https://httpbin.org/get?key=value");
    }

    #[tokio::main]
    #[test]
    async fn test_post_params() {
        let resp = post_params().await.unwrap();

        assert!(resp.args.get("key").is_none());
        assert_eq!(resp.url, "https://httpbin.org/post");
        assert_eq!(resp.json["body"], "json");
    }
}
