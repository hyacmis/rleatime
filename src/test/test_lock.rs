#[cfg(test)]
mod testss {
    use crate::xuexi::gaoji::fn_lock;

    #[test]
    fn test_fn1() {
        fn_lock::lock1()
    }
    #[test]
    fn test_fn2() {
        fn_lock::lock2()
    }

    #[test]
    fn test_fn3() {
        fn_lock::lock3()
    }
}
