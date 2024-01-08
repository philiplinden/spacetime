use hifitime::{Duration, Epoch};

#[derive(Clone, Debug)]
pub struct Clock {
    // Observed time
    pub epoch: Epoch,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            epoch: Epoch::from_unix_seconds(0.),
        }
    }
}

impl Clock {
    pub fn new(epoch: Epoch) -> Self {
        Clock {
            epoch,
        }
    }
    pub fn tick(&mut self, elapsed: Duration) {
        self.epoch = self.epoch + elapsed
    }
}
