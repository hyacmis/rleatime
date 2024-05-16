#[cfg(test)]
mod test_chrono {
    use chrono::{
        DateTime, Datelike, Duration, Local, Months, NaiveDate, NaiveDateTime, TimeZone, Utc,
    };

    #[test]
    fn test_today() {
        let dt = Local::now();

        assert_eq!(dt.year(), 2024);
    }

    #[test]
    fn test_format() {
        let dt: DateTime<Local> = Local.with_ymd_and_hms(2024, 4, 23, 11, 40, 0).unwrap();

        let dt_str = dt.format("%Y-%m-%d %H:%M:%S").to_string();

        assert_eq!(dt_str, "2024-04-23 11:40:00");

        assert_eq!(dt.to_rfc3339(), "2024-04-23T11:40:00+08:00");
        assert_eq!(dt.to_rfc2822(), "Tue, 23 Apr 2024 11:40:00 +0800");
        assert_eq!(dt.to_utc(), dt);
        assert_eq!(dt.to_string(), "2024-04-23 11:40:00 +08:00");
    }

    #[test]
    fn test_parse() {
        let dt: DateTime<Local> = Local.with_ymd_and_hms(2024, 4, 23, 11, 40, 0).unwrap();

        let dt_str = "2024-04-23 11:40:00";
        let naive_dt = NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S").unwrap();
        // err: called `Result::unwrap()` on an `Err` value: ParseError(Invalid)
        // let dt_parse :NaiveDateTime = dt_str.parse().unwrap();

        let local_dt: DateTime<Local> = Local.from_local_datetime(&naive_dt).unwrap();

        assert_eq!(local_dt, dt);

        let rfc3339 = DateTime::parse_from_rfc3339("2024-04-23T11:40:00+08:00").unwrap();
        let rfc3339_local: DateTime<Local> = rfc3339.into();

        assert_eq!(rfc3339_local, dt);

        let timestamp = 1713843600;
        let ts_date: DateTime<Utc> = DateTime::from_timestamp(timestamp, 0).unwrap();

        assert_eq!(ts_date.to_string(), "2024-04-23 03:40:00 UTC");

        let local_ts_date: DateTime<Local> = DateTime::from(ts_date);
        assert_eq!(local_ts_date.to_string(), "2024-04-23 11:40:00 +08:00");
    }

    #[test]
    fn test_date_op() {
        let dt = NaiveDate::from_ymd_opt(2024, 4, 23).unwrap();

        let add_str = dt + Duration::days(1);
        assert_eq!(add_str.to_string(), "2024-04-24");

        let sub_str = dt + Duration::days(-1);
        assert_eq!(sub_str.to_string(), "2024-04-22");

        let add_month_str = dt + Months::new(1);
        assert_eq!(add_month_str.to_string(), "2024-05-23");

        // 通过months 实现加一年, 是否还有其他方法?
        let add_year_str = dt + Months::new(12);
        assert_eq!(add_year_str.to_string(), "2025-04-23");
    }
}
