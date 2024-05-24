#![allow(dead_code)]

use chrono::{DateTime, Local};
use std::time::Duration;

use crate::serialize::serde_repr::Gender;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Animal {
    name: String,
    age: u32,
    like: &'static str,
    time: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cat {
    name: String,
    age: u32,
    color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
    pet_phrase: Vec<String>,
    pets: Cat,
    // #[serde(with = "ts_seconds")] // 使用秒级精度序列化日期时间
    birthday: DateTime<Local>,
}

#[cfg(test)]
mod test_to_json {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_basic_structure() {
        let animal = Animal {
            name: "Tom".to_string(),
            age: 3,
            like: "Jerry",
            time: Duration::from_secs(8),
        };
        let json = serde_json::to_string(&animal).unwrap();
        assert_eq!(
            json,
            r#"{"name":"Tom","age":3,"like":"Jerry","time":{"secs":8,"nanos":0}}"#
        );
    }

    #[test]
    fn test_more_structure() {
        let pp = vec!["hello world".to_string(), "perf_code.com".to_string()];
        let p = Person {
            name: "ho".to_string(),
            age: 18,
            gender: Gender::Male,
            pet_phrase: pp,
            pets: Cat {
                name: "Tom".to_string(),
                age: 2,
                color: Color::Red,
            },
            birthday: Local.with_ymd_and_hms(2024, 4, 23, 11, 40, 0).unwrap(),
        };

        // 序列化为JSON字符串
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(
            json,
            r#"{"name":"ho","age":18,"gender":1,"pet_phrase":["hello world","perf_code.com"],"pets":{"name":"Tom","age":2,"color":"Red"},"birthday":"2024-04-23T11:40:00+08:00"}"#
        );
    }
}
