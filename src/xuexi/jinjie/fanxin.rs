#![allow(dead_code)]

use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// 实现泛型特征的加法
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

///普通的泛型方法
// pub fn fx_add<T: Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
#[cfg(test)]
mod fan_xin_tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Point::new(12.4, 232.1);
        let b = Point::new(1.4, 3.6);
        let add_res = a + b;
        let add_answer = Point::new(13.8, 235.7);
        assert_eq!(add_answer, add_res);
    }

    #[test]
    fn test_sub() {
        let a = Point::new(12.4, 232.1);
        let b = Point::new(1.4, 3.6);
        let add_res = a - b;
        let add_answer = Point::new(11.0, 228.5);
        assert_eq!(add_answer, add_res);
    }
}
