#![allow(dead_code)]
pub fn t() {
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = b'a'; // 将字符'a'转换为整数，97

    if a < b as i8 {
        // as 转换
        println!("Ten is less than one hundred.");
    }
    println!("{a},{b},{c}");

    let c_: i8 = b.try_into().unwrap();

    if a < c_ {
        println!("{a}<{c_}");
    }
}
