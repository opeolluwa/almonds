use std::collections::HashSet;
use std::sync::{Mutex, RwLock};

pub struct SchedulerState {
    /// Keys of reminders already fired: "<uuid>-<fire_minute>".
    pub fired_keys: Mutex<HashSet<String>>,
    /// Lead time in minutes before remind_at to fire the alarm.
    pub lead_time_minutes: RwLock<i64>,
    /// Default alarm sound filename (None = silent).
    pub default_sound: RwLock<Option<String>>,
}

impl SchedulerState {
    pub fn new() -> Self {
        Self {
            fired_keys: Mutex::new(HashSet::new()),
            lead_time_minutes: RwLock::new(5),
            default_sound: RwLock::new(None),
        }
    }
}
