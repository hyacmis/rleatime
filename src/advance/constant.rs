#![allow(dead_code)]

use std::sync::atomic::{AtomicU8, Ordering};

const MAX_ID: u8 = 64;

#[derive(Debug, Eq, PartialEq)]
struct Factory {
    factory_id: u8,
}

static GLOBAL_ID_COUNTER: AtomicU8 = AtomicU8::new(0);

fn generate_id() -> u8 {
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
        Self {
            factory_id: generate_id(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Config {
    a: String,
    b: String,
}
static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    Some(Box::leak(c)) //Box::leak方法，它可以将一个变量从内存中泄漏
}

#[cfg(test)]
mod test_const {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(MAX_ID, 64);
    }

    #[test]
    fn test_update() {
        // 静态变量原则上不允许修改, 只允许在 `unsafe` 块内进行修改和使用
        static mut REQUEST_RECV: usize = 0;
        //必须使用unsafe语句块才能访问和修改static变量
        unsafe {
            REQUEST_RECV += 1;
            assert_eq!(REQUEST_RECV, 1)
        }
    }

    ///全局 ID 生成器
    #[test]
    fn test_global_id_counter() {
        let factory = Factory::new();
        assert_eq!(factory, Factory { factory_id: 0 })
    }
    #[test]
    fn test_box_leak() {
        let cfg = init();
        assert_eq!(
            *cfg.unwrap(),
            Config {
                a: "A".to_string(),
                b: "B".to_string(),
            }
        )
    }
}
