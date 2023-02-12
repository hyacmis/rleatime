mod test;
mod xuexi;

fn main() {
    // test::test_1::test1();
    //
    // let [a, b, c, d] = [1, 2, 3, 4];
    // println!("{a},{b},{c},{d}");
    //
    // let rect1 = xuexi::Rectangle::new(30, 50);
    // println!("{}",rect1.area());
    //
    // let m = xuexi::Message::Write(String::from("hello"));
    // m.call();
    //
    // let a = xuexi::Point::new(5,10);
    // let b = xuexi::Point::new(2.4,3.2);
    //
    // println!("add i32: {}", xuexi::fx_add(a.x,a.y));
    // println!("add f64: {}", xuexi::fx_add(b.x,b.y));

    // let post = xuexi::Post {
    //     title: "Rust语言简介".to_string(),
    //     author: "Sunface".to_string(),
    //     content: "Rust棒极了!".to_string(),
    // };
    // let weibo = xuexi::Weibo {
    //     username: "rust_arch".to_string(),
    //     content: "好像微博没Tweet好用".to_string(),
    // };
    // use xuexi::Summary;
    //
    // println!("{}", post.summarize());
    // println!("{}", weibo.summarize());
    //
    // let a = xuexi::Pair::new(3, 2);
    // a.cmp_display();
    //
    // let x = 1.1f64;
    // // do_something(&x);
    // let y = 8u8;
    //
    // // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    // xuexi::draw1(Box::new(x));
    // // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    // xuexi::draw1(Box::new(y));
    //
    // println!("{}", weibo.summarize());
    //
    // xuexi::draw2(&x);
    // xuexi::draw2(&y);

    // xuexi::ver_t::v()
    xuexi::map::mapping()
}
