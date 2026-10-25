use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerStatus {
    Idle,
    Running,
    Paused,
    Triggered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerEvent {
    Tick,
    Triggered,
}

#[derive(Debug)]
struct TimerInner {
    status: TimerStatus,
    duration: Duration,
    remaining: Duration,
    last_instant: Option<Instant>,
}

impl TimerInner {
    fn tick(&mut self) -> Option<TimerEvent> {
        if self.status != TimerStatus::Running {
            return None;
        }

        let last = self.last_instant?;
        let elapsed = last.elapsed();

        if elapsed >= Duration::from_secs(1) {
            let elapsed_secs = elapsed.as_secs();
            if self.remaining.as_secs() > elapsed_secs {
                self.remaining -= Duration::from_secs(elapsed_secs);
                self.last_instant = Some(Instant::now());
                Some(TimerEvent::Tick)
            } else {
                self.remaining = Duration::ZERO;
                self.status = TimerStatus::Triggered;
                self.last_instant = None;
                Some(TimerEvent::Triggered)
            }
        } else {
            None
        }
    }
}

/// Thread-safe Timer that can be safely shared across the UI and background keepawake threads.
#[derive(Debug, Clone)]
pub struct Timer {
    inner: Arc<Mutex<TimerInner>>,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(TimerInner {
                status: TimerStatus::Idle,
                duration: Duration::ZERO,
                remaining: Duration::ZERO,
                last_instant: None,
            })),
        }
    }

    #[allow(dead_code)]
    pub fn status(&self) -> TimerStatus {
        self.inner.lock().unwrap().status
    }

    #[allow(dead_code)]
    pub fn is_idle(&self) -> bool {
        self.inner.lock().unwrap().status == TimerStatus::Idle
    }

    pub fn is_running(&self) -> bool {
        self.inner.lock().unwrap().status == TimerStatus::Running
    }

    pub fn is_paused(&self) -> bool {
        self.inner.lock().unwrap().status == TimerStatus::Paused
    }

    pub fn is_active(&self) -> bool {
        let s = self.inner.lock().unwrap().status;
        s == TimerStatus::Running || s == TimerStatus::Paused
    }

    pub fn is_triggered(&self) -> bool {
        self.inner.lock().unwrap().status == TimerStatus::Triggered
    }

    #[allow(dead_code)]
    pub fn remaining_seconds(&self) -> u64 {
        self.inner.lock().unwrap().remaining.as_secs()
    }

    pub fn formatted_time(&self) -> String {
        let total_secs = self.inner.lock().unwrap().remaining.as_secs();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}:{:02}", hours, mins, secs)
    }

    pub fn start(&self, duration: Duration) -> bool {
        if duration.is_zero() {
            return false;
        }
        let mut inner = self.inner.lock().unwrap();
        inner.duration = duration;
        inner.remaining = duration;
        inner.status = TimerStatus::Running;
        inner.last_instant = Some(Instant::now());
        true
    }

    pub fn pause(&self) {
        let mut inner = self.inner.lock().unwrap();
        if inner.status == TimerStatus::Running {
            inner.tick();
            inner.status = TimerStatus::Paused;
            inner.last_instant = None;
        }
    }

    pub fn resume(&self) {
        let mut inner = self.inner.lock().unwrap();
        if inner.status == TimerStatus::Paused {
            inner.status = TimerStatus::Running;
            inner.last_instant = Some(Instant::now());
        }
    }

    pub fn reset(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.status = TimerStatus::Idle;
        inner.duration = Duration::ZERO;
        inner.remaining = Duration::ZERO;
        inner.last_instant = None;
    }

    pub fn tick(&self) -> Option<TimerEvent> {
        let mut inner = self.inner.lock().unwrap();
        inner.tick()
    }

    #[cfg(test)]
    pub fn set_remaining_for_test(&self, duration: Duration) {
        let mut inner = self.inner.lock().unwrap();
        inner.remaining = duration;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_initial_state() {
        let timer = Timer::new();
        assert_eq!(timer.status(), TimerStatus::Idle);
        assert_eq!(timer.remaining_seconds(), 0);
        assert_eq!(timer.formatted_time(), "00:00:00");
        assert!(!timer.is_running());
        assert!(!timer.is_active());
        assert!(timer.is_idle());
    }

    #[test]
    fn test_timer_start_zero_duration_fails() {
        let timer = Timer::new();
        assert!(!timer.start(Duration::ZERO));
        assert_eq!(timer.status(), TimerStatus::Idle);
    }

    #[test]
    fn test_timer_start_valid() {
        let timer = Timer::new();
        assert!(timer.start(Duration::from_secs(3665)));
        assert_eq!(timer.status(), TimerStatus::Running);
        assert_eq!(timer.remaining_seconds(), 3665);
        assert_eq!(timer.formatted_time(), "01:01:05");
        assert!(timer.is_running());
        assert!(timer.is_active());
    }

    #[test]
    fn test_timer_pause_and_resume() {
        let timer = Timer::new();
        timer.start(Duration::from_secs(100));

        timer.pause();
        assert_eq!(timer.status(), TimerStatus::Paused);
        assert!(timer.is_active());
        assert!(!timer.is_running());

        timer.resume();
        assert_eq!(timer.status(), TimerStatus::Running);
        assert!(timer.is_running());
    }

    #[test]
    fn test_timer_reset() {
        let timer = Timer::new();
        timer.start(Duration::from_secs(100));
        timer.reset();

        assert_eq!(timer.status(), TimerStatus::Idle);
        assert_eq!(timer.remaining_seconds(), 0);
        assert_eq!(timer.formatted_time(), "00:00:00");
    }

    #[test]
    fn test_formatted_time_cases() {
        let timer = Timer::new();
        timer.set_remaining_for_test(Duration::from_secs(0));
        assert_eq!(timer.formatted_time(), "00:00:00");

        timer.set_remaining_for_test(Duration::from_secs(59));
        assert_eq!(timer.formatted_time(), "00:00:59");

        timer.set_remaining_for_test(Duration::from_secs(3600));
        assert_eq!(timer.formatted_time(), "01:00:00");

        timer.set_remaining_for_test(Duration::from_secs(3600 * 12 + 60 * 34 + 56));
        assert_eq!(timer.formatted_time(), "12:34:56");
    }
}
