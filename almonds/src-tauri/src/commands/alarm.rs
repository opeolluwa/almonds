use std::io::BufReader;
use std::time::Duration;
use std::sync::atomic::Ordering;

use serde::Serialize;
use tauri::{AppHandle, Manager, State};
use ts_rs::TS;

use crate::{errors::AppError, state::alarm::AlarmState, state::scheduler::SchedulerState};

const SUPPORTED_EXTENSIONS: &[&str] = &["mp3", "wav", "ogg", "flac"];

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct AlarmSound {
    pub name: String,
    pub filename: String,
}

#[tauri::command]
pub fn list_alarm_sounds(app: AppHandle) -> Result<Vec<AlarmSound>, AppError> {
    let resource_dir = app.path().resource_dir().map_err(AppError::path)?;

    let mut sounds = Vec::new();

    for entry in std::fs::read_dir(&resource_dir).map_err(AppError::io)? {
        let entry = entry.map_err(AppError::io)?;
        let path = entry.path();

        let is_audio = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| SUPPORTED_EXTENSIONS.contains(&e))
            .unwrap_or(false);

        if is_audio {
            let filename = path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let name = path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            sounds.push(AlarmSound { name, filename });
        }
    }

    sounds.sort_by(|a, b| a.filename.cmp(&b.filename));

    Ok(sounds)
}

#[tauri::command]
pub fn play_alarm_sound(
    app: AppHandle,
    alarm_state: State<'_, AlarmState>,
    filename: String,
) -> Result<(), AppError> {
    let resource_dir = app.path().resource_dir().map_err(AppError::path)?;
    let path = resource_dir.join(&filename);

    if !path.exists() {
        return Err(AppError::io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Sound file not found: {filename}"),
        )));
    }

    let stop_flag = alarm_state.next_stop_flag();

    std::thread::spawn(move || {
        let (_stream, handle) = match rodio::OutputStream::try_default() {
            Ok(v) => v,
            Err(e) => {
                log::error!("Failed to open audio output: {e}");
                return;
            }
        };

        let sink = match rodio::Sink::try_new(&handle) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Failed to create audio sink: {e}");
                return;
            }
        };

        let file = match std::fs::File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Failed to open sound file: {e}");
                return;
            }
        };

        let source = match rodio::Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to decode sound file: {e}");
                return;
            }
        };

        sink.append(source);

        while !sink.empty() && !stop_flag.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(50));
        }

        sink.stop();
    });

    Ok(())
}

#[tauri::command]
pub fn stop_alarm_sound(alarm_state: State<'_, AlarmState>) {
    alarm_state.stop();
}

/// Called by the frontend whenever alarm settings change so the Rust scheduler
/// always uses the latest lead-time and default sound.
#[tauri::command]
pub fn set_alarm_settings(
    scheduler_state: State<'_, SchedulerState>,
    lead_time_minutes: i64,
    default_sound: Option<String>,
) {
    *scheduler_state.lead_time_minutes.write().unwrap() = lead_time_minutes;
    *scheduler_state.default_sound.write().unwrap() = default_sound;
}
