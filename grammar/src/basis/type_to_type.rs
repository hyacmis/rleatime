#[test]
fn test_as() {
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    assert!(a < b as i8);
}

#[test]
fn t_as_ptr() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
}

/// 当你为 U 实现 `From<T>` 之后，你要确保这个转换一定能成功，
/// 如若有失败的可能，你应该选择为 U 实现 `TryFrom<T>`。
/// 优先选择实现 `From<T>` 而不是 `Into<T>` ，
/// 因为当你为 U 实现 `From<T>` ，这意味着你同时也为 T 隐式实现了 `Into<U>`
#[test]
fn test_from() {
    #[derive(Debug)]
    struct Complex {
        re: i32,
        im: i32,
    }

    impl From<i32> for Complex {
        fn from(re: i32) -> Self {
            Complex { re, im: 0 }
        }
    }

    let b: Complex = 1.into();

    assert_eq!(b.re, 1);
    assert_eq!(b.im, 0);
}

#[test]
fn test_byte() {
    let a = 43.2324 as i32;
    let b = b'a';

    // 将字符'a'转换为整数，97
    let c: i32 = b.into();

    assert!(a < c);
}

#[test]
fn test_try_into() {
    let a: u8 = 10;
    let b: u16 = 1500;

    // 超过返回会引发 panic 错误
    let c = b.try_into().unwrap_or(255);

    assert!(a < c);
}
