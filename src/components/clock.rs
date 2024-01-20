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
    #[allow(dead_code)]
    pub fn new(epoch: Epoch) -> Self {
        Clock {
            epoch,
        }
    }
    #[allow(dead_code)]
    pub fn tick(&mut self, elapsed: Duration) {
        self.epoch = self.epoch + elapsed
    }
}
