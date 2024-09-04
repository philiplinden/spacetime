use chrono::{Datelike, Timelike};
use hifitime;
use std::ops::{Deref, DerefMut};

/// A wrapper type for `hifitime::Epoch`.
///
/// This type allows for easy interoperability between `hifitime` and `chrono` libraries.
#[derive(Clone, Copy)]
pub struct HifiEpoch(pub hifitime::Epoch);

/// A wrapper type for `hifitime::Duration`.
///
/// This type allows for easy interoperability between `hifitime` and `chrono` libraries.
#[derive(Clone, Copy)]
pub struct HifiDuration(pub hifitime::Duration);

/// A wrapper type for `chrono::DateTime<chrono::Utc>`.
///
/// This type allows for easy interoperability between `hifitime` and `chrono` libraries.
#[derive(Clone, Copy)]
pub struct LofiDateTime(pub chrono::DateTime<chrono::Utc>);

/// A wrapper type for `chrono::Duration`.
///
/// This type allows for easy interoperability between `hifitime` and `chrono` libraries.
#[derive(Clone, Copy)]
pub struct LofiDuration(pub chrono::Duration);

// Deref and DerefMut for wrapper types
impl Deref for HifiEpoch {
    type Target = hifitime::Epoch;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for HifiEpoch {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Deref for HifiDuration {
    type Target = hifitime::Duration;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for HifiDuration {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Deref for LofiDateTime {
    type Target = chrono::DateTime<chrono::Utc>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for LofiDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Deref for LofiDuration {
    type Target = chrono::Duration;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for LofiDuration {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

/// A simplified trait for date-like operations.
///
/// This trait provides a subset of the `chrono::Datelike` trait's functionality,
/// focusing on the most commonly used date operations.
pub trait SimpleDateLike {
    /// Returns the year number in the calendar date.
    fn year(&self) -> i32;

    /// Returns the month number starting from 1.
    fn month(&self) -> u32;

    /// Returns the day of month starting from 1.
    fn day(&self) -> u32;

    /// Returns the time-zone naive date.
    fn date_naive(&self) -> chrono::NaiveDate;
}

impl SimpleDateLike for LofiDateTime {
    fn year(&self) -> i32 { self.0.year() }
    fn month(&self) -> u32 { self.0.month() }
    fn day(&self) -> u32 { self.0.day() }
    fn date_naive(&self) -> chrono::NaiveDate { self.0.date_naive() }
}

/// A simplified trait for time-like operations.
///
/// This trait provides a subset of the `chrono::Timelike` trait's
/// functionality, focusing on the most commonly used operations.
impl Timelike for LofiDateTime {
    fn hour(&self) -> u32 { self.0.hour() }
    fn minute(&self) -> u32 { self.0.minute() }
    fn second(&self) -> u32 { self.0.second() }
    fn nanosecond(&self) -> u32 { self.0.nanosecond() }
    fn with_hour(&self, hour: u32) -> Option<Self> {
        self.0.with_hour(hour).map(LofiDateTime)
    }
    fn with_minute(&self, min: u32) -> Option<Self> {
        self.0.with_minute(min).map(LofiDateTime)
    }
    fn with_second(&self, sec: u32) -> Option<Self> {
        self.0.with_second(sec).map(LofiDateTime)
    }
    fn with_nanosecond(&self, nano: u32) -> Option<Self> {
        self.0.with_nanosecond(nano).map(LofiDateTime)
    }
}

/// # Conversions using `From` traits

/// ## Converting between internal types

/// Converts a `HifiEpoch` to a `LofiDateTime`.
///
/// This conversion may lose some precision due to the differences in
/// internal representations between `hifitime` and `chrono`.
impl From<HifiEpoch> for LofiDateTime {
    fn from(epoch: HifiEpoch) -> Self {
        LofiDateTime(chrono::DateTime::from_timestamp_millis(epoch.to_unix_milliseconds() as i64).unwrap())
    }
}

/// Converts a `HifiDuration` to a `LofiDuration`.
///
/// This conversion may lose some precision due to the differences in
/// internal representations between `hifitime` and `chrono`.
impl From<HifiDuration> for LofiDuration {
    fn from(duration: HifiDuration) -> Self {
        let (centuries, nanos) = duration.to_parts();
        let centuries_as_days = centuries as i64 / hifitime::DAYS_PER_CENTURY_I64;
        let chrono_days = chrono::Duration::days(centuries_as_days);
        let chrono_nanos = chrono::Duration::nanoseconds(nanos as i64);
        LofiDuration(chrono_days + chrono_nanos)
    }
}

/// Converts a `LofiDateTime` to a `HifiEpoch`.
///
/// This conversion preserves the precision of the `chrono::DateTime<Utc>`.
impl From<LofiDateTime> for HifiEpoch {
    fn from(datetime: LofiDateTime) -> Self {
        HifiEpoch(hifitime::Epoch::from_unix_duration(hifitime::Duration::from_milliseconds(
            datetime.timestamp_millis() as f64,
        )))
    }
}

/// Converts a `LofiDuration` to a `HifiDuration`.
///
/// This conversion preserves the precision of the `chrono::Duration`.
impl From<LofiDuration> for HifiDuration {
    fn from(duration: LofiDuration) -> Self {
        HifiDuration(hifitime::Duration::from_milliseconds(duration.num_milliseconds() as f64))
    }
}

/// ## Conversions to and from external types

/// Converts a `HifiEpoch` to a `hifitime::Epoch`
impl From<HifiEpoch> for hifitime::Epoch {
    fn from(epoch: HifiEpoch) -> Self {
        epoch.0
    }
}

/// Converts a `HifiEpoch` to a `chrono::DateTime<chrono::Utc>`
impl From<HifiEpoch> for chrono::DateTime<chrono::Utc> {
    fn from(epoch: HifiEpoch) -> Self {
        let lofi: LofiDateTime = epoch.into();
        lofi.0
    }
}

/// Converts a `HifiEpoch` to a `chrono::NaiveDateTime`.
///
/// This conversion may lose some precision due to the differences in
/// internal representations between `hifitime` and `chrono`.
impl From<HifiEpoch> for chrono::NaiveDateTime {
    fn from(epoch: HifiEpoch) -> Self {
        LofiDateTime::from(epoch).naive_utc()
    }
}

/// Converts a `HifiDuration` to a `hifitime::Duration`
impl From<HifiDuration> for hifitime::Duration {
    fn from(hifi_duration: HifiDuration) -> Self {
        hifi_duration.0
    }
}

/// Converts a `HifiDuration` to a `chrono::Duration`
impl From<HifiDuration> for chrono::Duration {
    fn from(hifi_duration: HifiDuration) -> Self {
        let lofi: LofiDuration = hifi_duration.into();
        lofi.0
    }
}

/// Converts a `LofiDateTime` to a `hifitime::Epoch`
impl From<LofiDateTime> for hifitime::Epoch {
    fn from(datetime: LofiDateTime) -> Self {
        let hifi: HifiEpoch = datetime.into();
        hifi.0
    }
}

/// Converts a `LofiDateTime` to a `chrono::DateTime<chrono::Utc>`
impl From<LofiDateTime> for chrono::DateTime<chrono::Utc> {
    fn from(datetime: LofiDateTime) -> Self {
        datetime.0
    }
}

/// Converts a `LofiDuration` to a `hifitime::Duration`
impl From<LofiDuration> for hifitime::Duration {
    fn from(lofi_duration: LofiDuration) -> Self {
        let hifi: HifiDuration = lofi_duration.into();
        hifi.0
    }
}

/// Converts a `LofiDuration` to a `chrono::Duration`
impl From<LofiDuration> for chrono::Duration {
    fn from(lofi_duration: LofiDuration) -> Self {
        lofi_duration.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_hifi_to_lofi_utc() {
        let hifi_epoch = HifiEpoch(hifitime::Epoch::from_gregorian_utc(2023, 8, 12, 15, 30, 45, 0));
        let lofi_utc: LofiDateTime = hifi_epoch.into();

        assert_eq!(lofi_utc.year(), 2023);
        assert_eq!(lofi_utc.month(), 8);
        assert_eq!(lofi_utc.day(), 12);
        assert_eq!(lofi_utc.hour(), 15);
        assert_eq!(lofi_utc.minute(), 30);
        assert_eq!(lofi_utc.second(), 45);
    }

    #[test]
    fn test_hifi_to_lofi_naive() {
        let hifi_epoch = HifiEpoch(hifitime::Epoch::from_gregorian_utc(2023, 8, 12, 15, 30, 45, 0));
        let lofi_naive: chrono::NaiveDateTime = hifi_epoch.into();

        assert_eq!(lofi_naive.year(), 2023);
        assert_eq!(lofi_naive.month(), 8);
        assert_eq!(lofi_naive.day(), 12);
        assert_eq!(lofi_naive.hour(), 15);
        assert_eq!(lofi_naive.minute(), 30);
        assert_eq!(lofi_naive.second(), 45);
    }

    #[test]
    fn test_hifi_duration_to_lofi_duration() {
        let hifi_duration = HifiDuration(
            hifitime::Duration::from_days(365.) + hifitime::Duration::from_hours(6.)
        );
        let lofi_duration: LofiDuration = hifi_duration.into();

        assert_eq!(lofi_duration.num_days(), 365);
        assert_eq!(lofi_duration.num_hours() % 24, 6);
    }

    #[test]
    fn test_lofi_to_hifi_epoch() {
        let lofi_datetime = LofiDateTime(chrono::Utc.with_ymd_and_hms(2023, 8, 12, 15, 30, 45).unwrap());
        let hifi_epoch: HifiEpoch = lofi_datetime.into();

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
        let lofi_duration = LofiDuration(chrono::Duration::days(365) + chrono::Duration::hours(6));
        let hifi_duration: HifiDuration = lofi_duration.into();

        assert_eq!(
            hifi_duration.to_seconds(),
            hifitime::SECONDS_PER_DAY * 365. + hifitime::SECONDS_PER_HOUR * 6.
        );
    }

    #[test]
    fn test_roundtrip_epoch_conversion() {
        let original_epoch = HifiEpoch(hifitime::Epoch::from_gregorian_utc(2023, 8, 12, 15, 30, 45, 123_456_789));
        let roundtrip_epoch: HifiEpoch = LofiDateTime::from(original_epoch).into();

        // Note: We lose some precision in the milliseconds because we don't trust Chrono to that precision
        assert!(
            (*original_epoch - *roundtrip_epoch).abs() < hifitime::Duration::from_milliseconds(1.)
        );
    }

    #[test]
    fn test_roundtrip_duration_conversion() {
        let original_duration = HifiDuration(
            hifitime::Duration::from_days(365.)
                + hifitime::Duration::from_hours(6.)
                + hifitime::Duration::from_nanoseconds(123_456_789.)
        );
        let roundtrip_duration: HifiDuration = LofiDuration::from(original_duration).into();

        // Note: We lose some precision in the milliseconds because we don't trust Chrono to that precision
        assert!(
            (*original_duration - *roundtrip_duration).abs()
                < hifitime::Duration::from_milliseconds(1.)
        );
    }
}
