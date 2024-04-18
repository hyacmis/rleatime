#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use tokio::sync::Semaphore;

//死锁 锁与主线程的生命周期一样长
fn lock_deadlock() {
    let m = Mutex::new(5);

    let mut num = m.lock().unwrap();
    *num = 6;
    // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
    drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
    println!("m = {:?}", m);

    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    drop(num1); // 手动 drop num1 ，观察打印结果的不同

    println!("m = {:?}", m);
}

/// Rc<T>/RefCell<T>用于单线程内部可变性
/// Arc<T>/Mutex<T>用于多线程内部可变性
static MUTEX1: Mutex<i64> = Mutex::new(0);
static MUTEX2: Mutex<i64> = Mutex::new(0);

#[tokio::main]
async fn lock_semaphore() {
    let semaphore = Arc::new(Semaphore::new(2));
    let mut join_handles = Vec::new();

    for i in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            //
            // 在这里执行任务...
            //
            println!("lock_semaphore:{i}");
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}

#[cfg(test)]
mod test_lock {
    use std::sync::{Condvar, MutexGuard, RwLock};
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;

    use super::*;

    /// 互斥锁 Mutex
    #[test]
    fn test_lock_mutex() {
        // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
        let m = Mutex::new(5);

        {
            // 获取锁，然后deref为`m`的引用
            // lock返回的是Result
            // m.lock()向m申请一个锁, 该方法会阻塞当前线程，直到获取到锁，
            // 因此当多个线程同时访问该数据时，只有一个线程能获取到锁，
            // 其它线程只能阻塞着等待，这样就保证了数据能被安全的修改！
            let mut num = m.lock().unwrap(); //返回一个智能指针MutexGuard<T>
            *num = 6;
            // 锁自动被drop
        }

        // 解锁并获取内部数据
        assert_eq!(*m.lock().unwrap(), 6);
    }

    /// 多线程中使用 Mutex
    #[test]
    fn test_lock_thread() {
        // 通过`Arc`实现`Mutex`的多所有权
        // Arc<T> 它的内部计数器是多线程安全的，因此可以在多线程环境中使用
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 2;
            });
            handles.push(handle);
        }

        // 等待所有子线程完成
        for handle in handles {
            handle.join().unwrap();
        }

        // 输出最终地计数结果
        assert_eq!(*counter.lock().unwrap(), 20);
    }

    /// 多线程死锁
    #[test]
    fn test_lock_thread_deadlock() {
        // 存放子线程的句柄
        let mut children = vec![];
        for i_thread in 0..2 {
            children.push(thread::spawn(move || {
                for _ in 0..1 {
                    // 线程1
                    if i_thread % 2 == 0 {
                        // 锁住MUTEX1
                        let _guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                        println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                        // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                        sleep(Duration::from_millis(10));

                        // 去锁MUTEX2
                        // let guard = MUTEX2.lock().unwrap();

                        let guard = MUTEX2.try_lock();
                        println!("线程 {} 获取 MUTEX2 锁的结果: {:?}", i_thread, guard);
                    } else {
                        // 锁住MUTEX2
                        let _guard = MUTEX2.lock().unwrap();

                        println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

                        // let _guard = MUTEX1.lock().unwrap();

                        sleep(Duration::from_millis(10));
                        let guard = MUTEX1.try_lock();
                        println!("线程 {} 获取 MUTEX1 锁的结果: {:?}", i_thread, guard);
                    }
                }
            }));
        }

        // 等子线程完成
        for child in children {
            let _ = child.join();
        }

        println!("死锁没有发生");
    }

    /// 读写锁
    /// 总结下RwLock:
    /// 同时允许多个读，但最多只能有一个写
    /// 读和写不能同时存在
    /// 读可以使用read、try_read，写write、try_write, 在实际项目中，try_xxx会安全的多
    #[test]
    fn test_lock_rwlock() {
        let lock = RwLock::new(5);

        // 同一时间允许多个读
        {
            let r1 = lock.read().unwrap();
            let r2 = lock.read().unwrap();
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        } // 读锁在此处被drop

        // 同一时间只允许一个写
        {
            let mut w = lock.write().unwrap();
            *w += 1;
            assert_eq!(*w, 6);

            // 以下代码会阻塞发生死锁，因为读和写不允许同时存在
            // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
            // let r1 = lock.read();
            // println!("{:?}", r1);
        } // 写锁在此处被drop
    }

    /// 用条件变量(Condvar)控制线程的同步
    #[test]
    fn test_lock_condvar() {
        let flag = Arc::new(Mutex::new(false));
        let cond = Arc::new(Condvar::new());
        let c_flag = flag.clone();
        let c_cond = cond.clone();

        let hdl = thread::spawn(move || {
            let mut lock = c_flag.lock().unwrap();
            let mut counter = 0;

            while counter < 3 {
                while !*lock {
                    // wait方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新。
                    // 同时当前线程在此处会被阻塞，直到被其他地方notify后，它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程。
                    lock = c_cond.wait(lock).unwrap();
                }

                *lock = false;

                counter += 1;
                println!("inner counter: {}", counter);
            }
        });

        let mut counter = 0;
        loop {
            sleep(Duration::from_millis(1000));
            *flag.lock().unwrap() = true;
            counter += 1;
            if counter > 3 {
                break;
            }
            println!("outside counter: {}", counter);
            cond.notify_one();
        }
        hdl.join().unwrap();

        assert!(*flag.lock().unwrap());
    }

    #[test]
    fn test_lock_semaphore() {
        lock_semaphore()
    }
}
