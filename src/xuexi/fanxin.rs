#![allow(dead_code)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self { Self { x, y } }
    // pub fn add(&self) -> T{
    //     fx_add(self.x + self.y) // 无法实现调用
    // }
}

pub fn fx_add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}