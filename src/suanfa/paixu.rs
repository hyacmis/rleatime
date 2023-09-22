// 选择排序
fn select_sort(vec: &mut Vec<i32>) {
    // 外循环需小于数组
    for i in 0..vec.len() - 1 {
        let mut min = i;
        // 开始 idx 大于外循环 小于数组长度
        for j in i + 1..vec.len() {
            // 判断后续值是否小于最小值
            if vec[min] > vec[j] {
                min = j; // 将已发现的小值赋给最小值
            }
        }
        // 当最小值是自己时跳过
        if min == i {
            continue;
        }
        (vec[i], vec[min]) = (vec[min], vec[i]); // 利用解包交换值
    }
}
#[test]
fn test_select_sort() {
    let mut vec = vec![2, 5, 27, 7, 13, 89, 21, 32, 64, 3];
    select_sort(&mut vec);
    assert_eq!(vec, vec![2, 3, 5, 7, 13, 21, 27, 32, 64, 89]);
    dbg!("{:#?}", vec);
}
