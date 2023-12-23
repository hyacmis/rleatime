#![allow(dead_code)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

impl User {
    pub fn new(active: bool, username: String, email: String, sign_in_count: u64) -> Self {
        Self {
            active,
            username,
            email,
            sign_in_count,
        }
    }
}

#[test]
fn test_user() {
    let user = User::new(
        true,
        String::from("John"),
        String::from("someone@example.com"),
        100,
    );
    assert_eq!(user.active, true);
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

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        // 在这里定义方法体
        println!("sadasdas")
    }
}
