#![allow(dead_code)]

use std::ops::{Add, Sub};

use chrono::{Datelike, Days, Duration, Months};

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

fn get_month_start<T>(dt: &T) -> T
where
    T: Datelike,
{
    dt.with_day(1).unwrap()
}

fn get_month_end<T>(dt: &T) -> T
where
    T: Datelike + Copy + Add<Months, Output = T> + Sub<Days, Output = T>,
{
    // 将其修改为1号
    let dt_temp = get_month_start(dt);
    // 加一个月 在减一天
    dt_temp + Months::new(1) - Days::new(1)
}

#[cfg(test)]
mod test_datetime_x {
    use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

    use super::*;

    struct DateType {
        naive_date: NaiveDate,
        naive_time: NaiveTime,
        naive_date_time: NaiveDateTime,
        local: DateTime<Local>,
        utc: DateTime<Utc>,
    }

    fn get_date(year: u32, month: u32, day: u32) -> DateType {
        let naive_date = NaiveDate::from_ymd_opt(year as i32, month, day).unwrap();
        let naive_time = NaiveTime::from_hms_opt(11, 40, 0).unwrap();
        let naive_date_time = NaiveDateTime::new(naive_date, naive_time);
        let local: DateTime<Local> = Local
            .with_ymd_and_hms(year as i32, month, day, 11, 40, 0)
            .unwrap();
        let utc: DateTime<Utc> = Utc
            .with_ymd_and_hms(year as i32, month, day, 11, 40, 0)
            .unwrap();

        DateType {
            naive_date,
            naive_time,
            naive_date_time,
            local,
            utc,
        }
    }

    #[test]
    fn test_get_week_start() {
        let dt = get_date(2024, 4, 23);

        let naive_date_result = get_week_start(&dt.naive_date);
        assert_eq!(naive_date_result.to_string(), "2024-04-22");

        let naive_date_time_result = get_week_start(&dt.naive_date_time);
        assert_eq!(naive_date_time_result.to_string(), "2024-04-22 11:40:00");

        let local_result = get_week_start(&dt.local);
        assert_eq!(local_result.to_string(), "2024-04-22 11:40:00 +08:00");

        let utc_result: DateTime<Utc> = get_week_start(&dt.utc);
        assert_eq!(utc_result.to_string(), "2024-04-22 11:40:00 UTC");
    }

    #[test]
    fn test_get_week_end() {
        let dt = get_date(2024, 4, 23);
        let naive_date_result = get_week_end(&dt.naive_date);
        assert_eq!(naive_date_result.to_string(), "2024-04-28");

        let naive_date_time_result = get_week_end(&dt.naive_date_time);
        assert_eq!(naive_date_time_result.to_string(), "2024-04-28 11:40:00");

        let local_result = get_week_end(&dt.local);
        assert_eq!(local_result.to_string(), "2024-04-28 11:40:00 +08:00");

        let utc_result: DateTime<Utc> = get_week_end(&dt.utc);
        assert_eq!(utc_result.to_string(), "2024-04-28 11:40:00 UTC");
    }

    #[test]
    fn test_get_month_start() {
        let dt = get_date(2024, 4, 23);

        let naive_date_result = get_month_start(&dt.naive_date);
        assert_eq!(naive_date_result.to_string(), "2024-04-01");

        let naive_date_time_result = get_month_start(&dt.naive_date_time);
        assert_eq!(naive_date_time_result.to_string(), "2024-04-01 11:40:00");

        let local_result = get_month_start(&dt.local);
        assert_eq!(local_result.to_string(), "2024-04-01 11:40:00 +08:00");

        let utc_result: DateTime<Utc> = get_month_start(&dt.utc);
        assert_eq!(utc_result.to_string(), "2024-04-01 11:40:00 UTC");
    }

    #[test]
    fn test_get_month_end() {
        let dt = get_date(2024, 2, 12);

        let naive_date_result = get_month_end(&dt.naive_date);
        assert_eq!(naive_date_result.to_string(), "2024-02-29");

        let naive_date_time_result = get_month_end(&dt.naive_date_time);
        assert_eq!(naive_date_time_result.to_string(), "2024-02-29 11:40:00");

        let local_result = get_month_end(&dt.local);
        assert_eq!(local_result.to_string(), "2024-02-29 11:40:00 +08:00");

        let utc_result: DateTime<Utc> = get_month_end(&dt.utc);
        assert_eq!(utc_result.to_string(), "2024-02-29 11:40:00 UTC");
    }
}
