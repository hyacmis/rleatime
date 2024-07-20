// 使用运算符时需要 使用借用(&)的方式,并且它会返回一个新的hashset
#[cfg(test)]
mod test_set {
    use std::collections::HashSet;

    #[test]
    #[allow(clippy::useless_vec)]
    fn test_intersection() {
        let a = vec![1, 2, 3, 4, 5]
            .iter()
            .cloned()
            .collect::<HashSet<i32>>();
        let b = vec![4, 5, 6, 7, 8]
            .iter()
            .cloned()
            .collect::<HashSet<i32>>();

        // 交集
        let mut intersection = a.intersection(&b).cloned().collect::<Vec<i32>>();
        intersection.sort();
        assert_eq!(intersection, [4, 5]);

        // 等同与上面
        let mut c = (&a & &b).iter().cloned().collect::<Vec<i32>>();
        c.sort();
        assert_eq!(c, [4, 5]);
    }

    #[test]
    fn test_union() {
        let a = HashSet::from([1, 2, 3, 4, 5]);
        let b = HashSet::from([4, 5, 6, 7, 8]);

        // 并集
        let mut union = a.union(&b).cloned().collect::<Vec<i32>>();
        union.sort();

        assert_eq!(union, [1, 2, 3, 4, 5, 6, 7, 8]);

        // 等同与上面
        let mut c = (&a | &b).iter().cloned().collect::<Vec<i32>>();
        c.sort();
        assert_eq!(c, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_diff() {
        let a = HashSet::from([1, 2, 3, 4, 5]);
        let b = HashSet::from([4, 5, 6, 7, 8]);

        // 差集
        let mut diff_a = a.difference(&b).cloned().collect::<Vec<i32>>();
        diff_a.sort();
        assert_eq!(diff_a, [1, 2, 3]);

        // 等同与上面
        let mut c = (&a - &b).iter().cloned().collect::<Vec<i32>>();
        c.sort();
        assert_eq!(c, [1, 2, 3]);

        let mut diff_b = b.difference(&a).cloned().collect::<Vec<i32>>();
        diff_b.sort();
        assert_eq!(diff_b, [6, 7, 8]);

        // 等同与上面
        let mut c = (&b - &a).iter().cloned().collect::<Vec<i32>>();
        c.sort();
        assert_eq!(c, [6, 7, 8]);
    }

    #[test]
    fn test_symmetric_difference() {
        let a = HashSet::from([1, 2, 3, 4, 5]);
        let b = HashSet::from([4, 5, 6, 7, 8]);

        // 对称差集
        let mut c = a.symmetric_difference(&b).cloned().collect::<Vec<i32>>();
        c.sort();

        assert_eq!(c, [1, 2, 3, 6, 7, 8]);

        // 等同与上面
        let mut c = (&a ^ &b).iter().cloned().collect::<Vec<i32>>();
        c.sort();
        assert_eq!(c, [1, 2, 3, 6, 7, 8]);
    }

    #[test]
    fn test_complement() {
        let a = HashSet::from([1, 2, 3, 4, 5]);
        let b = HashSet::from([4, 5, 6, 7, 8]);

        // 补集: 并集-交集
        let u = a.union(&b).cloned().collect::<HashSet<i32>>();
        let d = a.intersection(&b).cloned().collect::<HashSet<i32>>();
        let mut complement = u.difference(&d).cloned().collect::<Vec<i32>>();
        complement.sort();

        assert_eq!(complement, [1, 2, 3, 6, 7, 8])
    }

    #[test]
    fn test_is_disjoint() {
        let mut a = HashSet::from([1, 2, 3]);
        let b = HashSet::from([4, 5]);

        // 判断是否存在相同的元素,及是否存在交集
        assert!(a.is_disjoint(&b));
        a.insert(4);
        assert!(!a.is_disjoint(&b));
    }
}
