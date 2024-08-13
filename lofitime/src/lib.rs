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

impl HifiDuration for hifitime::Duration {
    fn to_lofi_duration(&self) -> chrono::Duration {
        let (centuries, nanos) = self.to_parts();
        let centuries_as_days = centuries as i64 / hifitime::DAYS_PER_CENTURY_I64;
        let chrono_days = chrono::Duration::days(centuries_as_days);
        let chrono_nanos = chrono::Duration::nanoseconds(nanos as i64);
        chrono_days + chrono_nanos
    }
}

/// Adds functions to a chrono time so it can be represented as a hifitime epoch.
pub trait LofiDateTime {
    fn to_hifi_epoch(&self) -> Result<hifitime::Epoch, hifitime::Errors>;
}

impl<Tz> LofiDateTime for chrono::DateTime<Tz>
where
    Tz: chrono::TimeZone,
{
    fn to_hifi_epoch(&self) -> Result<hifitime::Epoch, hifitime::Errors> {
        if let Some(utc_nanos) = self.to_utc().timestamp_nanos_opt() {
            Ok(hifitime::Epoch::from_utc_duration(
                hifitime::Duration::from_truncated_nanoseconds(utc_nanos),
            ))
        } else {
            Err(hifitime::Errors::Overflow)
        }
    }
}

/// Adds functions to a chrono duration so it can be represented as a hifitime duration.
pub trait LofiDuration {
    fn to_hifi_duration(&self) -> Result<hifitime::Duration, hifitime::Errors>;
}

impl LofiDuration for chrono::Duration {
    fn to_hifi_duration(&self) -> Result<hifitime::Duration, hifitime::Errors> {
        if let Some(nanos) = self.num_nanoseconds() {
            Ok(hifitime::Duration::from_truncated_nanoseconds(nanos))
        } else {
            Err(hifitime::Errors::Overflow)
        }
    }
}
