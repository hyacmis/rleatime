#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Copy)]
#[repr(u8)]
pub enum Gender {
    Male = 1,
    Female = 2,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct User {
    name: String,
    age: u8,
    gender: Gender,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Info {
    like: Option<String>,
    like_gender: Option<Gender>,
}

#[derive(Debug, Eq, PartialEq)]
struct InfoInt {
    like: Option<String>,
    like_gender: Option<u8>,
}

#[cfg(test)]
mod test_serde_repr {

    use super::*;

    #[test]
    fn test_from_str() {
        let json_str = r#"2"#;
        let t = serde_json::from_str::<Gender>(json_str).unwrap();
        assert_eq!(t, Gender::Female);
    }

    #[test]
    fn test_from_struct() {
        let json_str = r#"{"name":"Bob","age":25,"gender":1}"#;
        // 序列化时会直接将数字转为枚举
        let user_from_json: User = serde_json::from_str(json_str).unwrap();

        let user = User {
            name: "Bob".to_string(),
            age: 25,
            gender: Gender::Male,
        };
        assert_eq!(user_from_json, user)
    }

    #[test]
    fn test_as() {
        let t = Gender::Male;
        // 可直接 使用数字类型 的相应转换
        assert_eq!(t as u8, 1);
        assert_eq!(t as i32, 1);
        assert_eq!(t as usize, 1);
        assert_eq!(t as u64, 1);
    }

    #[test]
    fn test_option() {
        // Option 也会自动添加Some
        let json_str = r#"{"like":"haha","like_gender":1}"#;
        let info_from_json: Info = serde_json::from_str(json_str).unwrap();

        let info = Info {
            like: Some("haha".to_string()),
            like_gender: Some(Gender::Male),
        };

        assert_eq!(info_from_json, info);
        // 在 sea-orm 修改字段时,会遇到 类型转换的问题
        if info_from_json.like_gender.is_some() {
            let info_int = InfoInt {
                like: info_from_json.like,
                // 此处只能在 unwrap 后转换, 再手动添加Some来实现option
                like_gender: Some(info_from_json.like_gender.unwrap() as u8),
            };
            assert_eq!(
                info_int,
                InfoInt {
                    like: Some("haha".to_string()),
                    like_gender: Some(1),
                }
            )
        }
    }
}
