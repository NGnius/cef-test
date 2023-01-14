//! Miscellaneous utility functionality

/// Get the timestamp of right now in the local timezone
pub fn timestamp_now() -> String {
    chrono::offset::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, false)
}
