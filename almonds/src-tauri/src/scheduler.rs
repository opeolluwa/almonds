use std::io::BufReader;
use std::sync::atomic::Ordering;
use std::time::Duration;

use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::state::alarm::AlarmState;
use crate::state::app::AppState;
use crate::state::scheduler::SchedulerState;
use almond_kernel::repositories::reminder::ReminderRepositoryExt;

/// Spawned once at app startup. Wakes at every clock-minute boundary and fires
/// any reminders whose adjusted fire time (remind_at − lead_time) falls in the
/// current minute. Deduplicates via `SchedulerState::fired_keys`.
pub async fn run(app: AppHandle) {
    loop {
        // Sleep until the start of the next minute.
        let now = chrono::Utc::now();
        let secs_into_minute = now.timestamp() % 60;
        let secs_to_wait = if secs_into_minute == 0 {
            60
        } else {
            60 - secs_into_minute
        };
        tokio::time::sleep(Duration::from_secs(secs_to_wait as u64)).await;

        check_and_fire(&app).await;
    }
}

async fn check_and_fire(app: &AppHandle) {
    // Snapshot settings without holding the lock across await points.
    let (lead_time_minutes, default_sound) = {
        let sched = app.state::<SchedulerState>();
        let lead = *sched.lead_time_minutes.read().unwrap();
        let sound = sched.default_sound.read().unwrap().clone();
        (lead, sound)
    };

    let now = chrono::Utc::now();
    let now_minute = now.timestamp() / 60;
    let lead_duration = chrono::Duration::minutes(lead_time_minutes);

    let reminders = match app.state::<AppState>().reminder_repository.find_all().await {
        Ok(r) => r,
        Err(e) => {
            log::error!("[Scheduler] Failed to fetch reminders: {e}");
            return;
        }
    };

    for reminder in reminders {
        // Convert to UTC timestamp for timezone-agnostic comparison.
        let fire_at_ts = reminder.remind_at.timestamp() - lead_duration.num_seconds();
        let fire_minute = fire_at_ts / 60;

        if fire_minute != now_minute {
            continue;
        }

        let key = format!("{}-{}", reminder.identifier, fire_minute);

        {
            let sched = app.state::<SchedulerState>();
            let mut fired = sched.fired_keys.lock().unwrap();
            if fired.contains(&key) {
                continue;
            }
            fired.insert(key);
        }

        // OS notification.
        if let Err(e) = app
            .notification()
            .builder()
            .title("Reminder")
            .body(&reminder.title)
            .show()
        {
            log::error!("[Scheduler] Notification failed: {e}");
        }

        // Play sound (reminder-specific → user default → silent).
        let sound_file = reminder
            .alarm_sound
            .as_deref()
            .map(String::from)
            .or_else(|| default_sound.clone());

        let Some(filename) = sound_file else {
            continue;
        };

        let resource_dir = match app.path().resource_dir() {
            Ok(d) => d,
            Err(e) => {
                log::error!("[Scheduler] resource_dir: {e}");
                continue;
            }
        };

        let path = resource_dir.join(&filename);
        if !path.exists() {
            log::warn!("[Scheduler] Sound file not found: {}", path.display());
            continue;
        }

        let stop_flag = app.state::<AlarmState>().next_stop_flag();

        std::thread::spawn(move || {
            let (_stream, handle) = match rodio::OutputStream::try_default() {
                Ok(v) => v,
                Err(e) => {
                    log::error!("[Scheduler] Audio output: {e}");
                    return;
                }
            };
            let sink = match rodio::Sink::try_new(&handle) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("[Scheduler] Audio sink: {e}");
                    return;
                }
            };
            let file = match std::fs::File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    log::error!("[Scheduler] Open sound: {e}");
                    return;
                }
            };
            let source = match rodio::Decoder::new(BufReader::new(file)) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("[Scheduler] Decode sound: {e}");
                    return;
                }
            };

            sink.append(source);
            while !sink.empty() && !stop_flag.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(50));
            }
            sink.stop();
        });
    }
}
