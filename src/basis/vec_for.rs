#[cfg(test)]
mod test_vec {
    #[test]
    fn test_for_push() {
        // for 循环中动态添加值
        let mut result = Vec::new();
        for i in 0..5 {
            result.push(format!("string{i}"));
        }
        result[2] = "string_new".to_string();
        assert_eq!(
            result,
            vec!["string0", "string1", "string_new", "string3", "string4"]
        );
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn test_new() {
        let mut arr = vec![1];
        // 添加
        arr.push(1);
        assert_eq!(arr, vec![1, 1]);

        let mut arr1 = Vec::new();

        arr1.push(1);
        assert_eq!(arr1, vec![1]);

        let arr2 = [1, 2, 3, 4, 5];
        assert_eq!(arr2.to_vec(), vec![1, 2, 3, 4, 5]);

        // 从 from 中 转换出来地是 ASCII 码的10进制
        let arr_from = Vec::from("string");
        assert_eq!(arr_from, vec![115, 116, 114, 105, 110, 103]);

        let arr_from1 = Vec::from("12345");
        assert_eq!(arr_from1, vec![49, 50, 51, 52, 53]);
    }

    #[test]
    fn test_get() {
        let mut arr = [1, 2, 3, 4, 5];

        let third = arr[2];
        assert_eq!(third, 3);

        match arr.get(2) {
            Some(v) => arr[2] = v * 3,
            None => println!("去你的第三个元素，根本没有！"),
        }
        // 数组越界会报错
        // let does_not_exist = &v[100];
        let does_not_exist = arr.get(100);
        assert!(does_not_exist.is_none())
    }

    #[test]
    fn test_for_enumerate() {
        let arr: Vec<u8> = vec![1, 2, 3, 4, 5];
        // 同时获取数组的下标和值  注意v的权限
        for (idx, v) in arr.iter().enumerate() {
            assert_eq!(idx as u8 + 1, *v)
        }
    }
}
