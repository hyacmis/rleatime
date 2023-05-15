#[cfg(test)]
mod tests {
    use crate::xuexi::g_consts;

    #[test]
    fn name() {
        g_consts::fn_c1()
    }

    #[test]
    fn test_fn2() {
        let v = g_consts::fn_c2();
        assert_eq!(v, 1);
    }
    #[test]
    fn test_fn4() {
        g_consts::fn_c4();
    }
    
}
