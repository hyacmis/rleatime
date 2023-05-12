#![allow(unused_variables)]
#![allow(dead_code)]
pub fn if_func1() {
    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }
}

pub fn type_fresm() {
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = b'a'; // 将字符'a'转换为整数，97

    println!("{},{},{}", a, b, c);

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    println!("{} {}", values[1], 2);
}
