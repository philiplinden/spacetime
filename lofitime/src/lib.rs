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
        chrono::DateTime::from_timestamp_millis(self.to_unix_milliseconds() as i64).unwrap()
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
/// We only keep precision to the nearest millisecond from chrono.
pub trait LofiDateTime {
    fn to_hifi_epoch(&self) -> hifitime::Epoch;
}

impl<Tz> LofiDateTime for chrono::DateTime<Tz>
where
    Tz: chrono::TimeZone,
{
    fn to_hifi_epoch(&self) -> hifitime::Epoch {
        hifitime::Epoch::from_unix_duration(hifitime::Duration::from_milliseconds(
            self.to_utc().timestamp_millis() as f64,
        ))
    }
}

/// Adds functions to a chrono duration so it can be represented as a hifitime duration.
/// We only keep precision to the nearest millisecond from chrono.
pub trait LofiDuration {
    fn to_hifi_duration(&self) -> hifitime::Duration;
}

impl LofiDuration for chrono::TimeDelta {
    fn to_hifi_duration(&self) -> hifitime::Duration {
        hifitime::Duration::from_milliseconds(self.num_milliseconds() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono;
    use hifitime;

    #[test]
    fn test_hifi_to_lofi_utc() {
        let hifi_epoch = hifitime::Epoch::from_gregorian_utc(2023, 8, 12, 15, 30, 45, 0);
        let lofi_utc = hifi_epoch.to_lofi_utc();

        assert_eq!(chrono::Datelike::year(&lofi_utc), 2023);
        assert_eq!(chrono::Datelike::month(&lofi_utc), 8);
        assert_eq!(chrono::Datelike::day(&lofi_utc), 12);
        assert_eq!(chrono::Timelike::hour(&lofi_utc), 15);
        assert_eq!(chrono::Timelike::minute(&lofi_utc), 30);
        assert_eq!(chrono::Timelike::second(&lofi_utc), 45);
    }

    #[test]
    fn test_hifi_to_lofi_naive() {
        let hifi_epoch = hifitime::Epoch::from_gregorian_utc(2023, 8, 12, 15, 30, 45, 0);
        let lofi_naive = hifi_epoch.to_lofi_naive();

        assert_eq!(chrono::Datelike::year(&lofi_naive), 2023);
        assert_eq!(chrono::Datelike::month(&lofi_naive), 8);
        assert_eq!(chrono::Datelike::day(&lofi_naive), 12);
        assert_eq!(chrono::Timelike::hour(&lofi_naive), 15);
        assert_eq!(chrono::Timelike::minute(&lofi_naive), 30);
        assert_eq!(chrono::Timelike::second(&lofi_naive), 45);
    }

    #[test]
    fn test_hifi_duration_to_lofi_duration() {
        let hifi_duration =
            hifitime::Duration::from_days(365.) + hifitime::Duration::from_hours(6.);
        let lofi_duration = hifi_duration.to_lofi_duration();

        assert_eq!(lofi_duration.num_days(), 365);
        assert_eq!(lofi_duration.num_hours() % 24, 6);
    }

    #[test]
    fn test_lofi_to_hifi_epoch() {
        use chrono::TimeZone;

        let lofi_datetime = chrono::Utc.with_ymd_and_hms(2023, 8, 12, 15, 30, 45).unwrap();
        let hifi_epoch = lofi_datetime.to_hifi_epoch();

        let (y, m, d, h, min, s, _) = hifi_epoch.to_gregorian_utc();
        assert_eq!(y, 2023);
        assert_eq!(m, 8);
        assert_eq!(d, 12);
        assert_eq!(h, 15);
        assert_eq!(min, 30);
        assert_eq!(s, 45);
    }

    #[test]
    fn test_lofi_duration_to_hifi_duration() {
        let lofi_duration = chrono::Duration::days(365) + chrono::Duration::hours(6);
        let hifi_duration = lofi_duration.to_hifi_duration();

        assert_eq!(
            hifi_duration.to_seconds(),
            hifitime::SECONDS_PER_DAY * 365. + hifitime::SECONDS_PER_HOUR * 6.
        );
    }

    #[test]
    fn test_roundtrip_epoch_conversion() {
        let original_epoch =
            hifitime::Epoch::from_gregorian_utc(2023, 8, 12, 15, 30, 45, 123_456_789);
        let roundtrip_epoch = original_epoch.to_lofi_utc().to_hifi_epoch();

        // Note: We lose some precision in the milliseconds because we don't trust Chrono to that precision
        assert!(
            (original_epoch - roundtrip_epoch).abs() < hifitime::Duration::from_milliseconds(1.)
        );
    }

    #[test]
    fn test_roundtrip_duration_conversion() {
        let original_duration = hifitime::Duration::from_days(365.)
            + hifitime::Duration::from_hours(6.)
            + hifitime::Duration::from_nanoseconds(123_456_789.);
        let roundtrip_duration = original_duration.to_lofi_duration().to_hifi_duration();

        // Note: We lose some precision in the milliseconds because we don't trust Chrono to that precision
        assert!(
            (original_duration - roundtrip_duration).abs()
                < hifitime::Duration::from_milliseconds(1.)
        );
    }
}
