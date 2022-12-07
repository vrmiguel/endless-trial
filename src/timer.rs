use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    interval: Duration,
    last_ticked: Instant,
}

impl Timer {
    pub fn start_now_with_interval(interval: Duration) -> Self {
        Self {
            interval,
            last_ticked: Instant::now(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.last_ticked.elapsed() >= self.interval
    }

    pub fn reset(&mut self) {
        self.last_ticked = Instant::now()
    }

    pub fn elapsed(&self) -> Duration {
        self.last_ticked.elapsed()
    }
}
