#![allow(dead_code)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn new(active: bool, username: String, email: String, sign_in_count: u64) -> Self {
        Self {
            active,
            username,
            email,
            sign_in_count,
        }
    }
}

/// 需添加 `pub` 关键字 来让外部可访问
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
/// 元组结构体 用来实现类如 `颜色的rgb值`, `坐标点` 等带有一定顺序的结构体
struct Color(i32, i32, i32);

/// 单元结构体 完全不关心内部字段, 只关心它的行为
#[derive(Debug, Eq, PartialEq)]
struct AlwaysEqual;

// 假设我们定义一个自定义trait，它有一个方法返回bool值
trait MyTrait {
    fn check_string(s: &str) -> bool;
}

// 为它实现实现自定义trait
impl MyTrait for AlwaysEqual {
    fn check_string(s: &str) -> bool {
        s.contains('.')
    }
}

#[cfg(test)]
mod test_struct {
    use super::*;

    #[test]
    fn test_user() {
        let user = User::new(
            true,
            String::from("John"),
            String::from("someone@example.com"),
            100,
        );
        assert!(user.active);
        assert_eq!(user.username, String::from("John"));
        assert_eq!(user.email, String::from("someone@example.com"));
        assert_eq!(user.sign_in_count, 100);

        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("iris"),
            ..user
        };
        assert_eq!(user1.email, String::from("someone@example.com"))
    }

    #[test]
    fn test_tuple() {
        let color = Color(1, 3, 4);
        // 使用下标的方式调用内部的元素
        assert_eq!(color.0, 1)
    }

    #[test]
    fn test_unit() {
        let f = AlwaysEqual::check_string("a.b");
        assert!(f);

        let f2 = AlwaysEqual::check_string("abc");
        assert!(!f2);
    }
}
