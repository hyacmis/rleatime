#![allow(dead_code)]

const MAX_ID: usize = usize::MAX / 2;

pub fn fn_c1() {
    println!("用户ID允许的最大值是{}", MAX_ID);
}

static mut REQUEST_RECV: usize = 0;
pub fn fn_c2() -> usize {
    //必须使用unsafe语句块才能访问和修改static变量
    unsafe {
        REQUEST_RECV += 1;
        println!("{}", REQUEST_RECV);
        REQUEST_RECV
    }
}

pub fn fn_c3() {
    //全局 ID 生成器:
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Factory {
        factory_id: usize,
    }

    static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    const MAX_ID: usize = usize::MAX / 2;

    fn generate_id() -> usize {
        // 检查两次溢出，否则直接加一可能导致溢出
        let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
        if current_val > MAX_ID {
            panic!("Factory ids overflowed");
        }
        let next_id = GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        if next_id > MAX_ID {
            panic!("Factory ids overflowed");
        }
        next_id
    }

    impl Factory {
        fn new() -> Self {
            Self { factory_id: generate_id() }
        }
    }
}

pub fn fn_c4() {
    #[derive(Debug)]
    struct Config {
        a: String,
        b: String,
    }
    static mut CONFIG: Option<&mut Config> = None;

    fn init() -> Option<&'static mut Config> {
        let c = Box::new(Config { a: "A".to_string(), b: "B".to_string() });

        Some(Box::leak(c)) //Box::leak方法，它可以将一个变量从内存中泄漏
    }

    unsafe {
        CONFIG = init();

        println!("{:?}", CONFIG)
    }
}
