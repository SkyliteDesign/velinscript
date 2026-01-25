pub struct DateStdlib;

impl DateStdlib {
    pub fn generate_now_code() -> String {
        "crate::stdlib::datetime::DateTimeStdlib::now_timestamp()".to_string()
    }

    pub fn generate_format_code(timestamp: &str, format: &str) -> String {
        format!(
            "crate::stdlib::datetime::DateTimeStdlib::format_custom({}, {})",
            timestamp, format
        )
    }

    pub fn generate_add_days_code(timestamp: &str, days: &str) -> String {
        format!("({} as i64 + {} * 86400) as u64", timestamp, days)
    }

    pub fn generate_diff_code(ts1: &str, ts2: &str) -> String {
        format!("({} as i64 - {} as i64)", ts1, ts2)
    }

    pub fn generate_add_hours_code(timestamp: &str, hours: &str) -> String {
        format!("({} as i64 + {} * 3600) as u64", timestamp, hours)
    }

    pub fn generate_add_minutes_code(timestamp: &str, minutes: &str) -> String {
        format!("({} as i64 + {} * 60) as u64", timestamp, minutes)
    }

    pub fn generate_format_relative_code(timestamp: &str) -> String {
        format!(
            "{{
                let diff = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64 - {} as i64;
                if diff < 60 {{ format!(\"vor {{}} Sekunden\", diff) }}
                else if diff < 3600 {{ format!(\"vor {{}} Minuten\", diff / 60) }}
                else if diff < 86400 {{ format!(\"vor {{}} Stunden\", diff / 3600) }}
                else {{ format!(\"vor {{}} Tagen\", diff / 86400) }}
            }}",
            timestamp
        )
    }

    pub fn generate_is_weekend_code(timestamp: &str) -> String {
        format!(
            "{{
                let dt = chrono::DateTime::<chrono::Utc>::from_timestamp({} as i64, 0).unwrap_or_default();
                let weekday = dt.weekday();
                weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun
            }}",
            timestamp
        )
    }

    pub fn generate_is_weekday_code(timestamp: &str) -> String {
        format!(
            "{{
                let dt = chrono::DateTime::<chrono::Utc>::from_timestamp({} as i64, 0).unwrap_or_default();
                let weekday = dt.weekday();
                weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun
            }}",
            timestamp
        )
    }
}
