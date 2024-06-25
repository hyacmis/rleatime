#![allow(dead_code)]
use crate::senior::reference::weak::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

/// Weak
/// - 不计数
/// - 不拥有所有权
/// - 不阻止值被释放(drop)
/// - 引用的值存在返回 Some，不存在返回 None
/// - 通过 upgrade 取到 Option<Rc<T>>，然后再取值
///
/// 可访问，但没有所有权，不增加引用计数，因此不会影响被引用值的释放回收
/// 可由 Rc<T> 调用 downgrade 方法转换成 Weak<T>
/// Weak<T> 可使用 upgrade 方法转换成 Option<Rc<T>>，如果资源已经被释放，则 Option 的值是 None
/// 常用于解决循环引用的问题
///
/// 适用场景:
/// 1. 持有一个 Rc 对象的临时引用，并且不在乎引用的值是否依然存在
/// 2. 阻止 Rc 导致的循环引用，因为 Rc 的所有权机制，会导致多个 Rc 都无法计数归零

#[derive(Debug, PartialEq)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[cfg(test)]
mod test_weak {
    use super::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    fn test_error_circular_reference() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        //a的初始化rc计数
        assert_eq!(Rc::strong_count(&a), 1);

        //a指向的节点
        assert_eq!(a.tail(), Some(&RefCell::new(Rc::new(Nil))));
        // 创建`b`到`a`的引用
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        //在b创建后，a的rc计数
        assert_eq!(Rc::strong_count(&a), 2);

        //b的初始化rc计数
        assert_eq!(Rc::strong_count(&b), 1);

        //b指向的节点
        assert_eq!(
            b.tail(),
            Some(&RefCell::new(Rc::new(Cons(5, RefCell::new(Rc::new(Nil))))))
        );

        // 利用RefCell的可变性，创建了`a`到`b`的引用
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        //在更改a后，b的rc计数
        assert_eq!(Rc::strong_count(&b), 2);

        //在更改a后，a的rc计数
        assert_eq!(Rc::strong_count(&a), 2);

        // 下面一行println!将导致循环引用
        // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
        // println!("a next item = {:?}", a.tail());
    }

    #[test]
    fn test_circular_ref() {
        // 创建Rc，持有一个值5
        let five = Rc::new(5);

        // 通过Rc，创建一个Weak指针
        let weak_five = Rc::downgrade(&five);

        // Weak引用的资源依然存在，取到值5
        let strong_five: Option<Rc<_>> = weak_five.upgrade();
        assert_eq!(*strong_five.unwrap(), 5);

        // 手动释放资源`five`
        drop(five);

        // Weak引用的资源已不存在，因此返回None
        let strong_five: Option<Rc<_>> = weak_five.upgrade();
        assert_eq!(strong_five, None);
    }

    #[test]
    fn test_tools() {
        // 主人
        struct Owner {
            name: String,
            gadgets: RefCell<Vec<Weak<Gadget>>>,
        }

        // 工具
        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
        }

        // 创建一个 Owner
        // 需要注意，该 Owner 也拥有多个 `gadgets`
        let gadget_owner: Rc<Owner> = Rc::new(Owner {
            name: "Gadget Man".to_string(),
            gadgets: RefCell::new(Vec::new()),
        });

        // 创建工具，同时与主人进行关联：创建两个 gadget，他们分别持有 gadget_owner 的一个引用。
        let gadget1 = Rc::new(Gadget {
            id: 1,
            owner: gadget_owner.clone(),
        });

        let gadget2 = Rc::new(Gadget {
            id: 2,
            owner: gadget_owner.clone(),
        });

        // 为主人更新它所拥有的工具
        // 因为之前使用了 `Rc`，现在必须使用 `Weak`，否则就会循环引用
        gadget_owner
            .gadgets
            .borrow_mut()
            .push(Rc::downgrade(&gadget1));

        gadget_owner
            .gadgets
            .borrow_mut()
            .push(Rc::downgrade(&gadget2));

        let list = gadget_owner
            .gadgets
            .borrow()
            .iter()
            // 遍历 gadget_owner 的 gadgets 字段
            .map(|x| {
                // gadget_opt 是一个 Weak<Gadget> 。
                // 因为 weak 指针不能保证他所引用的对象仍然存在。
                // 所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判断其所指向的对象是否存在。
                // 当然，Option 为 None 的时候这个引用原对象就不存在了。
                let gadget = x.upgrade().unwrap();
                (gadget.id, gadget.owner.name.to_owned())
            })
            .collect::<Vec<(i32, String)>>();

        assert_eq!(
            list,
            vec![(1, "Gadget Man".to_string()), (2, "Gadget Man".to_string())]
        );

        // 在函数的最后，gadget_owner，gadget1 和 gadget2 都被销毁。
        // 具体是，因为这几个结构体之间没有了强引用（`Rc<T>`），所以，当他们销毁的时候。
        // 首先 gadget2 和 gadget1 被销毁。
        // 然后因为 gadget_owner 的引用数量为 0，所以这个对象可以被销毁了。
        // 循环引用问题也就避免了
    }
}
