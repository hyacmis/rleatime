#[cfg(test)]
mod test_from_any {
    use serde_json::Value;
    use std::str::from_utf8;

    #[test]
    fn test_from_str() {
        let json_str = r#"{"name":"code","age":18}"#;
        let p: Value = serde_json::from_str(json_str).unwrap();
        assert_eq!(p.get("name").unwrap().as_str().unwrap(), "code");
        assert_eq!(p.get("age").unwrap(), 18);
    }

    #[test]
    fn test_from_no_json() {
        let str = "404 Not Found".to_string();
        // serde_json 只能处理json格式的字符串,无法处理非json格式的字符串
        // 如果解析json格式失败使用 str::from_utf8 从字符串的字节形式转换为字符串
        let p = serde_json::from_str(&str)
            .or_else(|_| from_utf8(str.as_bytes()))
            .unwrap();
        assert_eq!(p, str);
    }

    #[test]
    fn test_from_bytes() {
        let json_bytes = r#"{"name":"code","age":18}"#.as_bytes();
        let p: Value = serde_json::from_slice(json_bytes).unwrap();
        assert_eq!(p.get("name").unwrap().as_str().unwrap(), "code");
        assert_eq!(p.get("age").unwrap(), 18);
    }

    #[test]
    fn test_from_array() {
        let json_array = r#"[1,2,3,4,5,6,7,8,9]"#;
        let p: Value = serde_json::from_str(json_array).unwrap();
        assert_eq!(p.as_array().unwrap().len(), 9);
        assert_eq!(p.as_array().unwrap().first().unwrap(), 1);
    }
}
