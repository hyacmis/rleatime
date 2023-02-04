mod test;
mod  xuexi;

fn main() {
    // test::test_1::test1();

    // let [a, b, c, d] = [1, 2, 3, 4];
    // println!("{a},{b},{c},{d}");
    
    let rect1 = xuexi::Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let m = xuexi::Message::Write(String::from("hello"));
    m.call();
}


