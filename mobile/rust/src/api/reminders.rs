use almond_kernel::{
    adapters::reminder::{CreateReminder, UpdateReminder},
    repositories::reminder::ReminderRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use chrono::DateTime;

use crate::error::{make_meta, parse_uuid, AppError};
use crate::state::app_state;

/// `remind_at` — RFC 3339 / ISO 8601 datetime string with timezone,
/// e.g. `"2026-04-10T09:00:00+01:00"`.
#[flutter_rust_bridge::frb]
pub async fn create_reminder(
    title: String,
    description: Option<String>,
    remind_at: String,
    recurring: bool,
    recurrence_rule: Option<String>,
    alarm_sound: Option<String>,
    workspace_identifier: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let ws_id = workspace_identifier
        .as_deref()
        .map(parse_uuid)
        .transpose()
        .map_err(|e: AppError| e.to_string())?;

    let remind_at = DateTime::parse_from_rfc3339(&remind_at)
        .map_err(|e| format!("invalid remind_at datetime: {e}"))?;

    let payload = CreateReminder {
        title,
        description,
        remind_at,
        recurring,
        recurrence_rule,
        alarm_sound,
        workspace_identifier: ws_id,
    };

    let reminder = app_state()
        .reminders
        .create(&payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&reminder).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_reminder(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<Option<String>, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let reminder = app_state()
        .reminders
        .find_by_id(&id, &meta)
        .await
        .map_err(|e| e.to_string())?;

    reminder
        .map(|r| serde_json::to_string(&r).map_err(|e| e.to_string()))
        .transpose()
}

#[flutter_rust_bridge::frb]
pub async fn get_all_reminders(meta_workspace_id: Option<String>) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let reminders = app_state()
        .reminders
        .find_all(&meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&reminders).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn update_reminder(
    identifier: String,
    title: Option<String>,
    description: Option<String>,
    remind_at: Option<String>,
    recurring: Option<bool>,
    recurrence_rule: Option<String>,
    alarm_sound: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let remind_at = remind_at
        .as_deref()
        .map(|s| DateTime::parse_from_rfc3339(s).map_err(|e| format!("invalid remind_at: {e}")))
        .transpose()?;

    let payload = UpdateReminder {
        title,
        description,
        remind_at,
        recurring,
        recurrence_rule,
        alarm_sound,
    };

    let reminder = app_state()
        .reminders
        .update(&id, &payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&reminder).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn delete_reminder(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<(), String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    app_state()
        .reminders
        .delete(&id, &meta)
        .await
        .map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn duplicate_reminder(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .reminders
        .duplicate_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn transfer_reminder(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .reminders
        .transfer_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}
