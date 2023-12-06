use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    pet_phrase: Vec<String>,
}

#[test]
fn person_to_struct() {
    // 一个&str类型的JSON数据
    let data = r#"
        {
            "name": "James Bond",
            "age": 33,
            "pet_phrase": [
                "Bond, James Bond.",
                "Shaken, not stirred."
            ]
        }"#;

    // 转换成 Person 结构
    let p: Person = serde_json::from_str(data).unwrap();

    // 通过方括号建立索引来访问部分数据
    println!(
        "NAME: {}\nAGE: {}\n\t{}\n\t{}",
        p.name, p.age, p.pet_phrase[0], p.pet_phrase[1],
    );
}

#[test]
fn to_struct_hong() {
    //json!宏来非常自然的构建serde_json::Value对象
    let info = json!(
        {
            "name": "哈哈",
            "age": 14,
            "pet_phrase":[
                "汪汪.",
                "喵喵."
                ]
        }
    );
    println!("{}", info.to_string())
}
