use indicatif::ProgressBar;

pub fn base() {
    let bar = ProgressBar::new(100);
    for _ in 0..100 {
        bar.inc(1);
        // ...
    }
    bar.finish();
}
#[cfg(test)]
mod test_indicatif {
    use crate::tui::indicatif::base;

    #[test]
    fn test_base() {
        base()
    }
}
