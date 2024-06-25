#[cfg(test)]
mod test_thread {
    use std::cell::RefCell;
    use std::sync::{Arc, Barrier, Condvar, Mutex, Once};
    use std::thread;
    use std::time::Duration;

    /// 每次打印结果顺序不一样 ==> 不能依赖线程的执行顺序
    /// 最基础的多线程执行方式
    #[test]
    fn test_base() {
        let handle = thread::spawn(|| {
            // 内部使用闭包执行
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        // 如果不添加此语句 会导致 handle 线程在主线程结束后终止程序,无法输出完整的数字
        // handle.join().unwrap(); //可以让当前线程阻塞，直到它等待的子线程的结束

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap(); // 放置在主线程下方,会让两个线程交替输出
    }

    /// move 将所有权从一个线程转移到另外一个线程
    #[test]
    fn test_move() {
        let v = vec![1, 2, 3];
        //线程的启动时间点和结束时间点是不确定的
        let handle = thread::spawn(move || {
            assert_eq!(v, vec![1, 2, 3]);
        });
        handle.join().unwrap();
        // 下面代码会报错borrow of moved value: `v`
        // println!("{:?}",v);
    }

    /// 线程屏障(Barrier) 让多个线程都执行到某个点后，才继续一起往后执行
    #[test]
    fn test_barrier() {
        let mut handles = Vec::with_capacity(6);
        let barrier = Arc::new(Barrier::new(6));

        for i in 0..6 {
            let b = barrier.clone();
            handles.push(thread::spawn(move || {
                println!("before wait {}", i);
                b.wait(); // 再次等待上面的打印语句全部输出后在打印下面的语句
                println!("after wait {}", i);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    /// 使用 thread_local 宏可以初始化线程局部变量，然后在线程内部使用该变量的 with 方法获取变量值：
    #[test]
    fn test_local() {
        thread_local!(static FOO: RefCell<u32> = const { RefCell::new(1) });
        // 线程局部变量 每个新的线程访问它时，都会使用它的初始值作为开始，各个线程中的 FOO 值彼此互不干扰
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
        });

        // 每个线程开始时都会拿到线程局部变量的FOO的初始值
        let t = thread::spawn(move || {
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            });
        });

        // 等待线程完成
        t.join().unwrap();

        // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 2);
        });
    }

    ///条件变量(Condition Variables)经常和 Mutex 一起使用，可以让线程挂起，直到某个条件发生后再继续执行
    #[test]
    fn test_condvar() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap(); //子线程获取到锁
            println!("sub thread waiting lock");
            *started = true;
            println!("sub thread acquire lock");
            cvar.notify_one(); //通知主线程
        });

        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            //如果没有获取到锁就一直等待获取锁
            println!("main thread waiting lock");
            started = cvar.wait(started).unwrap(); //此处挂起等待子线程,并释放了锁started
            println!("main thread acquire lock");
        }
    }

    #[test]
    fn test_once() {
        static mut VAL: usize = 0;
        //保证全局变量只会被初始化一次，随后的其它线程调用就会忽略该函数
        static INIT: Once = Once::new();

        //call_once 执行初始化过程一次，并且只执行一次。
        let handle1 = thread::spawn(move || {
            INIT.call_once(|| unsafe {
                VAL = 1;
            });
        });

        let handle2 = thread::spawn(move || {
            INIT.call_once(|| unsafe {
                VAL = 2;
            });
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
        // val 的值取决于哪个线程先调用 INIT.call_once
        println!("{}", unsafe { VAL });
    }
}
