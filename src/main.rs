mod test;
mod xuexi;

fn main() {
    // test::test_1::test1();

    // let [a, b, c, d] = [1, 2, 3, 4];
    // println!("{a},{b},{c},{d}");
    
    let rect1 = xuexi::Rectangle::new(30, 50);
    println!("{}",rect1.area());

    let m = xuexi::Message::Write(String::from("hello"));
    m.call();



    let a = xuexi::Point::new(5,10);
    let b = xuexi::Point::new(2.4,3.2);

    println!("add i32: {}", xuexi::fx_add(a.x,a.y));
    println!("add f64: {}", xuexi::fx_add(b.x,b.y));
}


