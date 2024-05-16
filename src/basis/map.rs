#[cfg(test)]
mod test_map {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_iter() {
        let teams_list = vec![
            ("中国队".to_string(), 100),
            ("美国队".to_string(), 10),
            ("日本队".to_string(), 50),
        ];
        // 将行值的k v 转为 map的k v 需要手动指定 类型标注
        let dict: HashMap<String, i32> = teams_list.into_iter().collect();

        // 创建一个包含预期键的 HashSet
        let expected_keys: HashSet<String> = [
            "日本队".to_string(),
            "中国队".to_string(),
            "美国队".to_string(),
        ]
        .into_iter()
        .collect();

        let keys_set = dict.keys().cloned().collect::<HashSet<_>>();

        // 比较实际 Map 的键集合与期望的键集合是否一致
        assert_eq!(keys_set, expected_keys);
    }

    #[test]
    fn test_get() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score: Option<&i32> = scores.get(&team_name);
        assert_eq!(*score.unwrap(), 10);
    }

    #[test]
    fn test_install() {
        let mut scores = HashMap::new();

        scores.insert("Blue", 10);
        // 覆盖已有的值
        let old = scores.insert("Blue", 20);
        assert_eq!(old, Some(10));

        // 查询新插入的值
        let new = scores.get("Blue");
        assert_eq!(new, Some(&20));

        // 查询Yellow对应的值，若不存在则插入新值
        let v = scores.entry("Yellow").or_insert(5);
        assert_eq!(*v, 5); // 不存在，插入5

        // 查询Yellow对应的值，若不存在则插入新值
        let v = scores.entry("Yellow").or_insert(50);
        assert_eq!(*v, 5); // 已经存在，因此50没有插入
    }

    /// 查询某个 key 对应的值，若不存在则插入新值，若存在则对已有的值进行更新
    #[test]
    fn test_get_key_value() {
        let text = "hello world wonderful world";

        let mut dict = HashMap::new();
        // 根据空格来切分字符串(英文单词都是通过空格切分)
        for word in text.split_whitespace() {
            // 通过插入默认值（如果为空）来确保值位于条目中， 并返回对条目中值的可变引用
            let count = dict.entry(word).or_insert(0);
            *count += 1;
        }
        assert_eq!(
            dict,
            HashMap::from([("hello", 1), ("world", 2), ("wonderful", 1)])
        );
    }

    #[test]
    fn test_contains_key() {
        let mut dict = HashMap::new();
        dict.insert("a", 1);
        dict.insert("b", 2);
        dict.insert("c", 3);
        // 判断 key 是否在 map 中
        let f = dict.contains_key("d");

        assert!(!f);
    }
}
