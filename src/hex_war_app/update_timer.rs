use std::time::{Duration, Instant};

pub struct UpdateTimer {
    last_update: Instant,
    delay: Duration,
}

impl UpdateTimer {
    pub fn new(fps: u32) -> Self {
        let delay_secs = 1f64 / fps as f64;
        let delay = Duration::from_secs_f64(delay_secs);
        let last_update = Instant::now();
        Self { last_update, delay }
    }

    pub fn is_time_to_update(&self) -> bool {
        let now = Instant::now();
        now > self.next_update()
    }

    pub fn next_update(&self) -> Instant {
        self.last_update + self.delay
    }

    pub fn update(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed_time = now - self.last_update;
        self.last_update = now;
        elapsed_time
    }

    pub fn elapsed_time(&self) -> Duration {
        Instant::now() - self.last_update
    }
}

#[cfg(test)]
mod tests {
    use crate::hex_war_app::update_timer::UpdateTimer;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn is_time_to_update() {
        let fps = 10;
        let timer = UpdateTimer::new(fps);
        let delay = Duration::from_secs_f64(1f64 / fps as f64);
        thread::sleep(delay / 2);
        assert!(!timer.is_time_to_update());
        thread::sleep(delay / 2);
        assert!(timer.is_time_to_update());
    }
}
