use app_error::ApplicationError;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

pub(crate) fn convert_timestamp_to_local(timestamp: String) -> Result<String, ApplicationError> {
    let naive_dt = NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| ApplicationError::Unexpected(e.to_string()))?;
    let local_time: DateTime<Local> = DateTime::from(Utc.from_utc_datetime(&naive_dt));
    Ok(local_time.format("%Y-%m-%d %H:%M:%S").to_string())
}
