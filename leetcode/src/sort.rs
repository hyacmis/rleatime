/// 选择排序
pub fn select_sort(vec: &mut [i32]) {
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
        vec.swap(i, min);
    }
}

/// 冒泡排序
pub fn bubble_sort(vec: &mut [i32]) {
    // 排序次数
    for i in 0..vec.len() - 1 {
        //实际比较
        for j in 0..vec.len() - i - 1 {
            //判断当前值和下一位值的大小
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}

/// 插入排序
pub fn insert_sort(vec: &mut [i32]) {
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

/// 快速排序
pub fn quick_sort(data: &mut [i32]) {
    quick_sort_helper(data, 1, data.len());
}

fn quick_sort_helper(data: &mut [i32], start: usize, end: usize) {
    if start < end {
        let left = partition(data, start, end);
        if data[left] < data[start - 1] {
            data.swap(left, start - 1);
        }

        quick_sort_helper(data, start, left);
        quick_sort_helper(data, left + 1, end);
    }
}

fn partition(data: &mut [i32], start: usize, end: usize) -> usize {
    let mut left = start;
    let mut right = end - 1;
    while left < right {
        while data[left] <= data[start - 1] && left < right {
            left += 1;
        }

        while data[right] >= data[start - 1] && left < right {
            right -= 1;
        }
        data.swap(left, right);
    }
    left
}

#[cfg(test)]
mod test_sorts {
    use super::*;

    fn get_arr() -> Vec<i32> {
        vec![2, 5, 27, 7, 13, 89, 21, 32, 64, 3]
    }

    #[test]
    fn test_select_sort() {
        let mut vec = get_arr();
        select_sort(&mut vec);
        assert_eq!(vec, vec![2, 3, 5, 7, 13, 21, 27, 32, 64, 89]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut vec = get_arr();
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![2, 3, 5, 7, 13, 21, 27, 32, 64, 89]);
    }

    #[test]
    fn test_insert_sort() {
        let mut vec = get_arr();
        insert_sort(&mut vec);
        assert_eq!(vec, vec![2, 3, 5, 7, 13, 21, 27, 32, 64, 89]);
    }

    #[test]
    fn test_quick_sort() {
        let mut vec = get_arr();
        quick_sort(&mut vec);
        assert_eq!(vec, vec![2, 3, 5, 7, 13, 21, 27, 32, 64, 89]);
    }
}
