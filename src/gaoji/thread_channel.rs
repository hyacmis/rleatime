#[cfg(test)]
mod test_channel {
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_mpsc() {
        // 创建一个消息通道, 返回一个元组：(发送者，接收者)
        let (tx, rx) = mpsc::channel(); //创建的通道是异步通道

        // 创建线程，并发送消息
        thread::spawn(move || {
            // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
            tx.send(1).unwrap();

            // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
            // tx.send(Some(1)).unwrap()
        });

        // 在主线程中接收子线程发送的消息并输出
        // 会阻塞当前线程，直到读取到值
        assert_eq!(rx.recv().unwrap(), 1);

        // try_recv方法并不会阻塞线程，当通道中没有消息时，它会立刻返回一个错误
        // println!("receive {:?}", rx.try_recv());
    }

    #[test]
    fn test_seed_mut() {
        let (tx, rx) = mpsc::channel();
        //传输具有所有权的数据
        // thread::spawn(move || {
        //     let s = String::from("我，飞走咯!"); // String 没有实现 Copy 方法
        //     // 变量 s 的所有权被转移走
        //     tx.send(s).unwrap();
        //     // println!("val is {}", s);  报错
        // });
        //
        // let received = rx.recv().unwrap();
        // println!("Got: {}", received);

        //使用 for 进行循环接收
        thread::spawn(move || {
            let vals = vec![
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        // 主线程和子线程是并发运行的

        let mut f = "1".to_string();

        for received in rx {
            // 此循环是阻塞的
            assert_eq!(received, f);
            let a = f.parse::<u8>().unwrap();
            f = (a + 1).to_string()
        }
    }

    /// 多发送者
    #[test]
    fn test_more_tx() {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone(); //子线程会拿走发送者的所有权,所以复制一份
        thread::spawn(move || {
            tx.send(String::from("hi from raw tx")).unwrap();
        });

        thread::spawn(move || {
            tx1.send(String::from("hi from cloned tx")).unwrap();
        });
        //输出顺序也不确定
        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn test_async() {
        // channel 创建的通道是异步通道
        let (tx, rx) = mpsc::channel();

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

    #[test]
    fn test_sync() {
        //指定同步通道的消息缓存条数
        let (tx, rx) = mpsc::sync_channel(1);

        let handle = thread::spawn(move || {
            println!("首次发送之前");
            tx.send(1).unwrap();
            println!("首次发送之后");
            tx.send(1).unwrap();
            println!("再次发送之后");
        });

        println!("睡眠之前");
        thread::sleep(Duration::from_secs(3));
        println!("睡眠之后");

        println!("receive {}", rx.recv().unwrap());
        handle.join().unwrap();
    }

    #[test]
    fn test_more_type() {
        enum Fruit {
            Apple(u8),
            Orange(String),
        }
        // 需要手动指定  channel 类型
        let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

        tx.send(Fruit::Orange("sweet".to_string())).unwrap();
        tx.send(Fruit::Apple(2)).unwrap();
        tx.send(Fruit::Apple(43)).unwrap();

        for _ in 0..3 {
            //recv 会一直等待channel传递的值
            match rx.recv().unwrap() {
                Fruit::Apple(count) => println!("received {} apples", count),
                Fruit::Orange(flavor) => println!("received {} oranges", flavor),
            }
        }
    }
}
