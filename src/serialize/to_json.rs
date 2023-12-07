use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Animal {
    name: String,
    age: u32,
}

#[test]
fn test_to_json() {
    let animal = Animal {
        name: "Tom".to_owned(),
        age: 3,
    };
    let json = serde_json::to_string(&animal).unwrap();
    let res = r#"{"name":"Tom","age":3}"#.to_string();
    assert_eq!(res, json);
    println!("{}", json); // {"name":"Tom","age":3}
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    pet_phrase: Vec<String>,
}

#[test]
fn person_to_json() {
    let pp = vec!["hello world".to_string(), "perfcode.com".to_string()];
    let p = Person {
        name: "ho".to_string(),
        age: 18,
        pet_phrase: pp,
    };

    // 序列化为JSON字符串
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(
        r#"{"name":"ho","age":18,"pet_phrase":["hello world","perfcode.com"]}"#,
        json
    );
    println!("{}", json);
}
