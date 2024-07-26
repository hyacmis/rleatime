use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Person {
    name: String,
    age: u8,
    pet_phrase: Vec<String>,
}

#[cfg(test)]
mod test_to_struct {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_from() {
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
        let _p2 = serde_json::from_str::<Person>(data).unwrap();

        // 通过方括号建立索引来访问部分数据
        let p_str = format!(
            "NAME: {}; AGE: {}; pet_phrase: [{},{}]",
            p.name, p.age, p.pet_phrase[0], p.pet_phrase[1],
        );
        assert_eq!(
            p_str,
            r#"NAME: James Bond; AGE: 33; pet_phrase: [Bond, James Bond.,Shaken, not stirred.]"#
        )
    }

    #[test]
    fn test_macros() {
        // json!宏会将其转换为serde_json::Value对象
        // 这会生成一个动态类型
        let info = json!({
            "name": "哈哈",
            "age": 14,
            "pet_phrase":[
                "汪汪.",
                "喵喵."
                ]
        });

        let res = Person {
            name: "哈哈".to_string(),
            age: 14,
            pet_phrase: vec!["汪汪.".to_string(), "喵喵.".to_string()],
        };

        // 从Value对象准换为Person对象
        let info_person: Person = serde_json::from_value(info).unwrap();

        assert_eq!(info_person, res)
    }

    #[test]
    fn test_string() {
        let str = String::from(r#"{"name":"嘿嘿","age":43,"pet_phrase":["哞哞","咩咩"]}"#);
        let info = serde_json::from_str::<Person>(&str).unwrap();

        let res = Person {
            name: "嘿嘿".to_string(),
            age: 43,
            pet_phrase: vec!["哞哞".to_string(), "咩咩".to_string()],
        };

        assert_eq!(info, res)
    }
}
