#![allow(dead_code)]

use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

fn today() -> Result<Duration, SystemTimeError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(now)
}

#[cfg(test)]
mod test_time {
    use super::*;

    #[test]
    fn test_today() {
        let now = today().unwrap();
        assert!(now.as_secs().to_string().contains("17"))
    }

    #[test]
    fn test_from() {
        let five_seconds = Duration::from_secs(5);
        assert_eq!(five_seconds, Duration::from_millis(5_000));
        assert_eq!(five_seconds, Duration::from_micros(5_000_000));
        assert_eq!(five_seconds, Duration::from_nanos(5_000_000_000));

        let ten_seconds = Duration::from_secs(10);
        let seven_nanos = Duration::from_nanos(7);
        let total = ten_seconds + seven_nanos;
        assert_eq!(total, Duration::new(10, 7));
    }

    #[test]
    fn test_op() {
        let five_seconds = Duration::new(5, 0);
        let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 15);

        assert_eq!(five_seconds_and_five_nanos.as_secs(), 5);
        assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), 15);

        let ten_secs = Duration::from_secs(10);

        let total = five_seconds + ten_secs;
        assert_eq!(total.as_secs(), 15);
    }
}
