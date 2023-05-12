#![allow(dead_code)]
use std::cell::RefCell;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Barrier, Condvar, Mutex, Once};
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

static mut VAL: usize = 0;
static INIT: Once = Once::new();
pub(crate) fn thread_fn6() {
    let handle1 = thread::spawn(move || {
        //执行初始化过程一次，并且只执行一次。
        //如果当前有另一个初始化过程正在运行，线程将阻止该方法被调用。
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

    println!("{}", unsafe { VAL });
}

pub(crate) fn sync1() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        for i in 0..10 {
            tx.send(i).unwrap();
        }

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 在主线程中接收子线程发送的消息并输出
    for i in rx {
        println!("receive {}", i);
    }
}

pub(crate) fn sync2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {:?}", rx.try_recv());
}

pub(crate) fn sync3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("我，飞走咯!");
        tx.send(s).unwrap();
        // 下面一行会报错, 原因是所有权被转移了
        // println!("val is {}", s); //borrow of moved value: `s` value borrowed here after move
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub(crate) fn sync4() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // 多线程发送需要 clone 发送者

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        // for 循环发送
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for i in 0..10 {
            tx1.send(format!("tx {i}")).unwrap();
        }
    });

    // for 循环接受
    for received in rx {
        println!("Got: {}", received);
    }
}

pub(crate) fn sync5() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        // for 循环发送
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
            for i in 0..5 {
                tx.send(format!("tx {i}")).unwrap();
            }
        }
    });
    // for 循环接受
    for received in &rx {
        println!("Got1: {}", received);
    }
    // 下方循环无法接受值 ?
    for received in rx {
        println!("Got2: {}", received);
    }
}

pub(crate) fn sync6() {
    let (tx, rx) = mpsc::channel(); // 默认异步通道

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

pub(crate) fn sync7() {
    let (tx, rx) = mpsc::sync_channel(0); //同步通道

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

enum Fruit {
    Apple(u8),
    Orange(String),
}
pub(crate) fn channel_ohter() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }
}

pub(crate) fn sync8() {
    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        //子线程拿走的是复制后的send的所有权
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }
    // 通道关闭的两个条件：发送者全部drop或接收者被drop
    // send自身没有被drop,导致该循环永远无法结束
    // 在这里drop send...
    drop(send);

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}
