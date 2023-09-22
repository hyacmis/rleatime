#![allow(dead_code)]

use std::cell::RefCell;
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;
use std::time::Duration;
pub(crate) fn thread_fn() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub(crate) fn thread_fn1() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); //handle.join，可以让当前线程阻塞

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub(crate) fn thread_fn2() {
    let v = vec![1, 2, 3];
    //move 来将所有权从一个线程转移到另外一个线程
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    // 下面代码会报错borrow of moved value: `v`
    // println!("{:?}",v);
}

// 线程屏障(Barrier) 让多个线程都执行到某个点后，才继续一起往后执行
pub(crate) fn thread_fn3() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
//使用 thread_local 宏可以初始化线程局部变量，然后在线程内部使用该变量的 with 方法获取变量值：
pub(crate) fn thread_fn4() {
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

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

//条件变量(Condition Variables)经常和 Mutex 一起使用，可以让线程挂起，直到某个条件发生后再继续执行
pub(crate) fn thread_fn5() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    dbg!(" starting ");

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap(); //此处挂起等待子线程,并释放了锁
    }

    println!("started changed");

    dbg!(" ending... ");
}
