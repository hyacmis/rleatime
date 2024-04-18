use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Person {
    name: String,
    age: u8,
    pet_phrase: Vec<String>,
}

#[cfg(test)]
mod test_json_to_struct {
    use serde_json::Value;

    use crate::serialize::to_struct::Person;

    #[test]
    fn test_person_to_struct() {
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
        let p_str = format!(
            "NAME: {}\nAGE: {}\n\t{}\n\t{}",
            p.name, p.age, p.pet_phrase[0], p.pet_phrase[1],
        );
        assert_eq!(
            p_str,
            r#"NAME: James Bond
AGE: 33
	Bond, James Bond.
	Shaken, not stirred."#
        )
    }

    #[test]
    fn test_hong() {
        use serde_json::json;
        //json!宏来非常自然的构建serde_json::Value对象
        let info: Value = json!({
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

        // 将 JSON 值解析为 Person 类型
        let info_person: Person = serde_json::from_value(info.clone()).unwrap();

        assert_eq!(info_person, res)
    }
}
