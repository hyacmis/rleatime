#![allow(dead_code)]

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

//冒泡排序
fn maopao_sort(vec: &mut Vec<i32>) {
    // 排序次数
    for i in 0..vec.len() - 1 {
        //实际比较
        for j in 0..vec.len() - i - 1 {
            //判断当前值和下一位值的大小
            if vec[j] > vec[j + 1] {
                (vec[j], vec[j + 1]) = (vec[j + 1], vec[j]);
            }
        }
    }
}

#[test]
fn test_maopao_sort() {
    let mut vec = vec![2, 5, 27, 7, 13, 89, 21, 32, 64, 3];
    maopao_sort(&mut vec);
    assert_eq!(vec, vec![2, 3, 5, 7, 13, 21, 27, 32, 64, 89]);
    dbg!("{:#?}", vec);
}

//插入排序
fn insert_sort(vec: &mut Vec<i32>) {
    for i in 0..vec.len() {
        let mut j = i;
        while j > 0 {
            if vec[j] < vec[j - 1] {
                vec.swap(j, j - 1);
            }
            j -= 1;
        }
    }
}
#[test]
fn test_insert_sort() {
    let mut vec = vec![2, 5, 27, 7, 13, 89, 21, 32, 64, 3];
    insert_sort(&mut vec);
    assert_eq!(vec, vec![2, 3, 5, 7, 13, 21, 27, 32, 64, 89]);
    dbg!("{:#?}", vec);
}
