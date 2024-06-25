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
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

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

    #[test]
    fn test_memory_order() {
        struct Counter {
            count: u64,
        }
        let n = Mutex::new(Counter { count: 0 });

        n.lock().unwrap().count += 1;

        assert_eq!(n.lock().unwrap().count, 1);

        let foo = AtomicU64::new(0);
        /*
        // Ordering 用于控制原子操作使用的内存顺序
        // Relaxed， 这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序
        // Release 释放，设定内存屏障(Memory barrier)，保证它之前的操作永远在它之前，
        //  但是它后面的操作可能被重排到它前面
        // Acquire 获取, 设定内存屏障，保证在它之后的访问永远在它之后，但是它之前的操作却有可能被重排到它后面，
        //  往往和Release在不同线程中联合使用
        // AcqRel, 是 Acquire 和 Release 的结合，同时拥有它们俩提供的保证。比如你要对一个 atomic 自增 1，
        //  同时希望该操作之前和之后的读取或写入操作不会被重新排序
        // SeqCst 顺序一致性， SeqCst就像是AcqRel的加强版，它不管原子操作是属于读取还是写入的操作，
        //  只要某个线程有用到SeqCst的原子操作，线程中该SeqCst操作前的数据操作绝对不会被重新排在该SeqCst操作之后，
        //  且该SeqCst操作后的数据操作也绝对不会被重新排在SeqCst操作前。
         */
        assert_eq!(foo.fetch_add(10, Ordering::Relaxed), 0);
        assert_eq!(foo.load(Ordering::SeqCst), 10);
    }

    /// Atomic 的应用场景
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
