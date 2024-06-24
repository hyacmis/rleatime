#![allow(dead_code)]
use std::cell::RefCell;

/// Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况
/// Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
///
/// RefCell:
/// - 与 Cell 用于可 Copy 的值不同，RefCell 用于引用
/// - RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
/// - RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
/// - 使用 RefCell 时，违背借用规则会导致运行期的 panic

// 定义在外部库中的特征
pub trait Messenger {
    fn send(&self, msg: String);
}

// --------------------------
// 我们的代码中的数据结构和实现
struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        // 无法借用不可变局部变量 `self.msg_cache` 作为可变变量
        // self.msg_cache.push(msg)

        self.msg_cache.borrow_mut().push(msg)
    }
}

#[cfg(test)]
mod cell_refcell {
    use super::{Messenger, MsgQueue};
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    #[test]
    fn test_cell() {
        let c = Cell::new("1234");
        let one = c.get();
        assert_eq!(one, "1234");

        c.set("5678");
        let two = c.get();
        assert_eq!(two, "5678");
    }

    /// #[test]
    /// fn test_error_cell() {
    ///     let c = Cell::new(String::from("1234"));
    ///     // Note: the following trait bounds were not satisfied: `std::string::String: std::marker::Copy`
    ///     let one = c.get();
    ///     assert_eq!(one, "1234");
    /// }

    #[test]
    #[should_panic]
    fn test_refcell() {
        let s = RefCell::new(String::from("hello, world"));
        let s1 = s.borrow();
        // 在编译期不会报任何错误, 但运行期会产生 panic
        let s2 = s.borrow_mut(); //already borrowed: BorrowMutError

        assert_eq!(s1.to_string(), "hello, world".to_string());
        assert_eq!(s2.to_string(), "hello, world".to_string());
    }

    #[test]
    fn test_mq() {
        let mq = MsgQueue {
            msg_cache: RefCell::new(Vec::new()),
        };
        mq.send("hello, world".to_string());

        assert_eq!(mq.msg_cache, RefCell::new(vec!["hello, world".to_string()]));
    }

    #[test]
    fn test_rc_and_refcell() {
        let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

        let s1 = s.clone();
        let s2 = s.clone();
        s2.borrow_mut().push_str(", oh yeah!");

        assert_eq!(s, s1);
        assert_eq!(s1, s2);
        assert_eq!(
            s2.borrow().to_owned(),
            "我很善变，还拥有多个主人, oh yeah!".to_string()
        );
    }
}
