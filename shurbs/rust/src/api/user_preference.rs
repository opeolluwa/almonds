use almond_kernel::{
    adapters::user_preference::{CreateUserPreference, UpdateUserPreference},
    repositories::user_preference::UserPreferenceRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};

use crate::error::{make_meta, parse_uuid};
use crate::state::app_state;

#[flutter_rust_bridge::frb]
pub async fn get_user_preference(
    meta_workspace_id: Option<String>,
) -> Result<Option<String>, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let pref = app_state()
        .user_preferences
        .get(&meta)
        .await
        .map_err(|e| e.to_string())?;

    pref.map(|p| serde_json::to_string(&p).map_err(|e| e.to_string()))
        .transpose()
}

#[flutter_rust_bridge::frb]
pub async fn create_user_preference(
    first_name: String,
    last_name: String,
    email: String,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;


    let payload = CreateUserPreference {
        first_name,
        last_name,
        email,
    };
    let pref = app_state()
        .user_preferences
        .create(&payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&pref).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn update_user_preference(
    identifier: String,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let payload = UpdateUserPreference {
        first_name,
        last_name,
        email,
    };

    let pref = app_state()
        .user_preferences
        .update(&id, &payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&pref).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn duplicate_user_preference(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .user_preferences
        .duplicate_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn transfer_user_preference(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .user_preferences
        .transfer_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}
