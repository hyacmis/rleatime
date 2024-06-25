#![allow(dead_code)]
/// https://course.rs/advance/concurrency-with-threads/send-sync.html
/// Send和Sync是 Rust 安全并发的重中之重，
/// 但是实际上它们只是标记特征(marker trait，该特征未定义任何行为，因此非常适合用于标记),
/// 实现Send的类型可以在线程间安全的传递其所有权
/// 实现Sync的类型可以在线程间安全的共享(通过引用)
///
/// 一个类型要在线程间安全的共享的前提是，指向它的引用必须能在线程间传递。
/// 因为如果引用都不能被传递，我们就无法在多个线程间使用引用去访问同一个数据了
/// 只要复合类型中有一个成员不是Send或Sync，那么该复合类型也就不是Send或Sync
/// 手动实现 Send 和 Sync 是不安全的
///

#[cfg(test)]
mod test_seed_sync {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_bare_pointer() {
        #[derive(Debug)]
        struct MyBox(*const u8);
        // 手动为其实现 这两个特征, 但必须使用 `unsafe`
        // 因为 rust 不为这个类型的线程安全负责, 需要程序员自己负责
        unsafe impl Send for MyBox {}
        unsafe impl Sync for MyBox {}

        let b = &MyBox(5 as *const u8);
        let v = Arc::new(Mutex::new(b));

        let t = thread::spawn(move || {
            // 我们访问的引用实际上还是对主线程中的数据的借用，转移进来的仅仅是外层的智能指针引用
            // 要解决需要为其实现 sync 即可
            let v1 = v.lock().unwrap();
            println!("{v1:?}");
        });

        t.join().unwrap();
    }
}
