use hifitime::{Duration, Epoch};
use uom::si::{
    f64::{Frequency, FrequencyDrift, Time},
    frequency::megahertz,
    frequency_drift::megahertz_per_second,
    time::{nanosecond, picosecond, second},
};

#[derive(Clone, Debug)]
pub struct Clock {
    // Observed time
    pub epoch: Epoch,
    // Oscillations per unit time
    frequency: Frequency,
    // Frequency deviation per unit time
    frequency_drift: FrequencyDrift,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            epoch: Epoch::from_unix_seconds(0.),
            frequency: Frequency::new::<megahertz>(10.0),
            frequency_drift: FrequencyDrift::new::<megahertz_per_second>(0.000_1),
        }
    }
}

impl Clock {
    pub fn new(epoch: Epoch) -> Self {
        Clock {
            epoch,
            frequency: Frequency::new::<megahertz>(10.0),
            frequency_drift: FrequencyDrift::new::<megahertz_per_second>(0.000_1),
        }
    }
    pub fn tick(&mut self, elapsed: Duration) {
        let true_elapsed: Time = duration_to_time(elapsed);
        let drift: Time = self.drift(true_elapsed);
        let observed_elapsed: Time = self.truncate(true_elapsed + drift);
        self.epoch = self.epoch + time_to_duration(observed_elapsed)
    }

    fn drift(&self, true_elapsed: Time) -> Time {
        1. / (true_elapsed * self.frequency_drift)
    }

    fn truncate(&self, t: Time) -> Time {
        let precision = (1. / self.frequency).get::<picosecond>();
        let measurement = t.get::<picosecond>();
        let nearest = (measurement.trunc() / precision.trunc()).ceil() * precision;
        Time::new::<picosecond>(nearest)
    }
}

fn duration_to_time(d: Duration) -> Time {
    Time::new::<second>(d.to_seconds())
}

fn time_to_duration(t: Time) -> Duration {
    Duration::from_nanoseconds(t.get::<nanosecond>())
}
