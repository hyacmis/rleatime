#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, JoinHandle};

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

/// Atomic的值具有内部可变性，你无需将其声明为mut
static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

#[cfg(test)]
mod test_atomic {
    use std::hint;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn test_atomic_const() {
        let mut threads = Vec::with_capacity(N_THREADS);

        for _ in 0..N_THREADS {
            threads.push(add_n_times(N_TIMES));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    }

    /// 无锁(lock free)数据结构
    /// 全局变量，例如全局自增 ID, 在后续章节会介绍
    /// 跨线程计数器，例如可以用于统计指标
    #[test]
    fn test_atomic_thread() {
        let spinlock = Arc::new(AtomicUsize::new(1));

        let spinlock_clone = Arc::clone(&spinlock);
        let thread = thread::spawn(move || {
            spinlock_clone.store(0, Ordering::SeqCst);
        });

        // 等待其它线程释放锁
        while spinlock.load(Ordering::SeqCst) != 0 {
            hint::spin_loop();
        }

        if let Err(panic) = thread.join() {
            println!("Thread had an error: {:?}", panic);
        }
        println!("{:?}", spinlock)
    }
}
