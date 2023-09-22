#[cfg(test)]
mod testss {
    use crate::xuexi::gaoji::bingfa;
    use crate::xuexi::gaoji::xianchen;

    #[test]
    fn test_fn() {
        xianchen::thread_fn()
    }

    #[test]
    fn test_fn1() {
        xianchen::thread_fn1()
    }
    #[test]
    fn test_fn2() {
        xianchen::thread_fn2()
    }
    #[test]
    fn test_fn3() {
        xianchen::thread_fn3()
    }
    #[test]
    fn test_fn4() {
        xianchen::thread_fn4()
    }

    #[test]
    fn test_fn5() {
        xianchen::thread_fn5()
    }

    #[test]
    fn test_fn6() {
        bingfa::thread_fn6()
    }

    #[test]
    fn test_sync1() {
        bingfa::sync1()
    }

    #[test]
    fn test_sync2() {
        bingfa::sync2()
    }

    #[test]
    fn test_sync4() {
        bingfa::sync4()
    }
    #[test]
    fn test_sync5() {
        bingfa::sync5()
    }
    #[test]
    fn test_sync6() {
        bingfa::sync6();
        println!("********************************");
        bingfa::sync7();
    }

    #[test]
    fn test_channel_ohter() {
        bingfa::channel_ohter();
    }

    #[test]
    fn test_sync8() {
        bingfa::sync8()
    }
}
