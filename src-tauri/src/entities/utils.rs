use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

pub(crate) fn convert_timestamp_to_local(timestamp: String) -> Option<String> {
    let naive_dt = match NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S") {
        Ok(dt) => dt,
        Err(_) => return None,
    };
    let local_time: DateTime<Local> = DateTime::from(Utc.from_utc_datetime(&naive_dt));
    Some(local_time.format("%Y-%m-%d %H:%M:%S").to_string())
}
