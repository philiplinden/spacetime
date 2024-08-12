use chrono;
use hifitime;

/// Adds functions to a hifitime epoch so it can be represented as a chrono time.
pub trait HifiDateTime {
    fn to_lofi_utc(&self) -> chrono::DateTime<chrono::Utc>;
    fn to_lofi_naive(&self) -> chrono::NaiveDateTime;
}

impl HifiDateTime for hifitime::Epoch {
    /// Represents an Epoch as a UTC date and time
    fn to_lofi_utc(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp_nanos(
            self.to_duration_in_time_scale(hifitime::TimeScale::UTC)
                .truncated_nanoseconds(),
        )
    }
    /// Represents an Epoch in UTC then strips it down to a naive time
    /// (no time zone).
    fn to_lofi_naive(&self) -> chrono::NaiveDateTime {
        self.to_lofi_utc().naive_utc()
    }
}

/// Adds functions to a hifitime duration so it can be represented as a chrono duration.
pub trait HifiDuration {
    fn to_lofi_duration(&self) -> chrono::Duration;
}

/// Adds functions to a chrono time so it can be represented as a hifitime epoch.
pub trait LofiDateTime {
    fn to_hifi_epoch(&self, timescale: hifitime::TimeScale) -> hifitime::Epoch;
    fn to_hifi_utc(&self) -> hifitime::Epoch;
}

/// Adds functions to a chrono duration so it can be represented as a hifitime duration.
pub trait LofiDuration {
    fn to_hifi_duration(&self) -> hifitime::Duration;
}
