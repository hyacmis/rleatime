#![allow(dead_code)]

use std::ops::{Add, Sub};

use chrono::{Datelike, Duration};

/// 获取到给定日期的周开始日期
/// 开始日期为周一
fn get_week_start<T>(dt: &T) -> T
where
    T: Datelike + Sub<Duration, Output = T> + Copy,
{
    // 先获取 dt.weekday() 的值
    let weekday = dt.weekday().num_days_from_monday() as i64;

    // 计算给定日期所在周的开始时间
    *dt - Duration::days(weekday)
}

/// 获取到给定日期的周结束日期
/// 结束日期为周日
fn get_week_end<T>(dt: &T) -> T
where
    T: Datelike + Add<Duration, Output = T> + Copy,
{
    // 获取 dt.weekday() 的值
    let current_weekday = dt.weekday().num_days_from_monday() as i32;

    // 计算到下个星期日所需的天数差
    let days_to_sunday = 6 - current_weekday;

    // 在给定日期上加上这些天数以得到周日日期
    *dt + Duration::days(days_to_sunday as i64)
}

#[cfg(test)]
mod test_datetime_x {
    use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

    use super::{get_week_end, get_week_start};

    #[test]
    fn test_get_week_start() {
        let dt_naive_date = NaiveDate::from_ymd_opt(2024, 4, 23).unwrap();
        let naive_date_result = get_week_start(&dt_naive_date);
        assert_eq!(naive_date_result.to_string(), "2024-04-22");

        let time = NaiveTime::from_hms_opt(11, 40, 0).unwrap();
        let dt_naive_date_time = NaiveDateTime::new(dt_naive_date, time);
        let naive_date_time_result = get_week_start(&dt_naive_date_time);
        assert_eq!(naive_date_time_result.to_string(), "2024-04-22 11:40:00");

        let dt_local: DateTime<Local> = Local.with_ymd_and_hms(2024, 4, 23, 11, 40, 0).unwrap();
        let local_result = get_week_start(&dt_local);
        assert_eq!(local_result.to_string(), "2024-04-22 11:40:00 +08:00");

        let dt_utc = Utc.with_ymd_and_hms(2024, 4, 23, 11, 40, 0).unwrap();
        let utc_result: DateTime<Utc> = get_week_start(&dt_utc);
        assert_eq!(utc_result.to_string(), "2024-04-22 11:40:00 UTC");
    }

    #[test]
    fn test_get_week_end() {
        let dt_naive_date = NaiveDate::from_ymd_opt(2024, 4, 23).unwrap();
        let naive_date_result = get_week_end(&dt_naive_date);
        assert_eq!(naive_date_result.to_string(), "2024-04-28");

        let time = NaiveTime::from_hms_opt(11, 40, 0).unwrap();
        let dt_naive_date_time = NaiveDateTime::new(dt_naive_date, time);
        let naive_date_time_result = get_week_end(&dt_naive_date_time);
        assert_eq!(naive_date_time_result.to_string(), "2024-04-28 11:40:00");

        let dt_local: DateTime<Local> = Local.with_ymd_and_hms(2024, 4, 23, 11, 40, 0).unwrap();
        let local_result = get_week_end(&dt_local);
        assert_eq!(local_result.to_string(), "2024-04-28 11:40:00 +08:00");

        let dt_utc = Utc.with_ymd_and_hms(2024, 4, 23, 11, 40, 0).unwrap();
        let utc_result: DateTime<Utc> = get_week_end(&dt_utc);
        assert_eq!(utc_result.to_string(), "2024-04-28 11:40:00 UTC");
    }
}
