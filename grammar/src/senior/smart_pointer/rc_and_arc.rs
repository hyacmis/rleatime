#![allow(dead_code)]
/// rc: reference counting 引用计数;
/// - 希望在堆上分配一个对象供程序的多个部分使用且无法确定哪个部分最后一个结束时，就可以使用 `Rc` 成为数据值的所有者
/// - `Rc/Arc` 是不可变引用，你无法修改它指向的值，只能进行读取，如果要修改，需要配合内部可变性 `RefCell` 或互斥锁 `Mutex`
/// - 一旦最后一个拥有者消失，则资源会自动被回收，这个生命周期是在编译期就确定下来的
/// - `Rc` 只能用于同一线程内部，想要用于线程之间的对象共享，你需要使用 `Arc`
/// - `Rc<T>` 是一个智能指针，实现了 `Deref` 特征，因此你无需先解开 `Rc` 指针，再使用里面的 T，而是可以直接使用 T

#[cfg(test)]
mod rc {
    use std::rc::Rc;

    #[test]
    fn test_rc() {
        let s = String::from("hello, world");
        // // s在这里被转移给a
        // let a = Box::new(s);
        // // 报错！此处继续尝试将 s 转移给 b
        // let b = Box::new(s);

        // 使用 Rc 就可以轻易解决：
        let a = Rc::new(s);
        // 这里的 `clone` 只复制了智能指针并增加了引用计数，并没有克隆底层数据
        // a, b是同一个智能指针的两个副本 共享了底层的字符串 s
        let b = Rc::clone(&a);

        assert_eq!(2, Rc::strong_count(&a));
        assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b))
    }

    // #[test]
    // fn test_thread_of_rc() {
    //     let s = Rc::new(String::from("多线程漫游者"));
    //     for _ in 0..5 {
    //         let _s = Rc::clone(&s);
    //         // error[E0277]: `Rc<String>` cannot be sent between threads safely
    //         // the trait `std::marker::Send` is not implemented for `Rc<String>`
    //         // Note: required because it's used within this closure
    //         // Note: required by a bound in `std::thread::spawn`
    //         let handle = thread::spawn(move || println!("{}", s));
    //
    //     }
    // }
}

/// arc: Atomic reference counting 原子化的 Rc<T> 智能指针
#[cfg(test)]
mod arc {
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_arc() {
        let s = Arc::new(String::from("多线程漫游者"));
        for _ in 0..10 {
            let s = Arc::clone(&s);
            let _handle = thread::spawn(move || println!("{}", s));
        }
    }
}
