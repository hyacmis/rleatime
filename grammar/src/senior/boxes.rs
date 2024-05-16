#![allow(dead_code)]

#[cfg(test)]
mod test_box {
    #[test]
    fn test_box_ownership() {
        // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
        let arr = Box::new([0; 1000]);
        // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
        // let arr1 = arr;
        // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
        // assert_eq!(arr.len(), 1000);

        // 使用clone() 才能将底层数据转移
        let arr1 = arr.clone();

        assert_eq!(arr.len(), 1000);

        assert_eq!(arr1.len(), 1000);
    }

    #[test]
    fn test_value() {
        let x = Box::new("1".to_string());
        let y = *x + "1";
        assert_eq!(y, "11".to_string());
    }
}
