use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

pub struct AlarmState {
    pub stop_flag: Mutex<Option<Arc<AtomicBool>>>,
}

impl AlarmState {
    pub fn new() -> Self {
        AlarmState {
            stop_flag: Mutex::new(None),
        }
    }

    /// Signals any currently playing sound to stop and returns a fresh stop flag
    /// for the next playback.
    pub fn next_stop_flag(&self) -> Arc<AtomicBool> {
        let mut guard = self.stop_flag.lock().unwrap();
        if let Some(prev) = guard.take() {
            prev.store(true, Ordering::Relaxed);
        }
        let flag = Arc::new(AtomicBool::new(false));
        *guard = Some(flag.clone());
        flag
    }

    pub fn stop(&self) {
        let guard = self.stop_flag.lock().unwrap();
        if let Some(flag) = guard.as_ref() {
            flag.store(true, Ordering::Relaxed);
        }
    }
}
