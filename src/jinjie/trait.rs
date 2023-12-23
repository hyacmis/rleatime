#![allow(dead_code)]

use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    // 默认方法
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    //  类似与 python的abs重写 继承者必须实现该方法
    // fn summarize_(&self) -> String;
}

pub struct Post {
    pub title: String,
    // 标题
    pub author: String,
    // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // 类似继承的写法
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
    // 必须实现才行
    // fn summarize_(&self) -> String {
    //     return format!("sdf");
    // }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.summarize_author(), self.content)
    }
}

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

pub trait DrawBox {
    fn draw(&self);
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
pub fn draw1(x: Box<dyn Draw>) {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

pub fn draw2(x: &dyn Draw) {
    x.draw();
}
