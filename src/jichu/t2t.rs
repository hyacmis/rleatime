#[cfg(test)]
mod test_type_to_type {
    #[test]
    fn test_num() {
        let a = 3.1 as i8;
        let b = 100_i8 as i32;
        assert!(a < b as i8);
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
}
