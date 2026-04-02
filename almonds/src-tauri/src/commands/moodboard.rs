use base64::{engine::general_purpose, Engine as _};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MoodboardImage {
    pub filename: String,
    pub src: String,
    pub title: String,
}

fn moodboard_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    Ok(app_data_dir.join("moodboard"))
}

#[tauri::command]
pub async fn save_moodboard_image(
    app: AppHandle,
    filename: String,
    bytes: Vec<u8>,
) -> Result<String, String> {
    let dir = moodboard_dir(&app)?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let ext = std::path::Path::new(&filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg")
        .to_lowercase();

    let unique_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let path = dir.join(&unique_name);

    std::fs::write(&path, bytes).map_err(|e| e.to_string())?;

    Ok(unique_name)
}

#[tauri::command]
pub async fn list_moodboard_images(app: AppHandle) -> Result<Vec<MoodboardImage>, String> {
    let dir = moodboard_dir(&app)?;

    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    entries.sort_by_key(|e| {
        e.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    entries.reverse();

    let mut images = vec![];

    for entry in entries {
        let path = entry.path();
        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let mime = match ext.as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            _ => "image/jpeg",
        };

        let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
        let data = general_purpose::STANDARD.encode(&bytes);

        images.push(MoodboardImage {
            filename: filename.clone(),
            src: format!("data:{};base64,{}", mime, data),
            title: filename,
        });
    }

    Ok(images)
}

#[tauri::command]
pub async fn delete_moodboard_image(app: AppHandle, filename: String) -> Result<(), String> {
    let path = moodboard_dir(&app)?.join(&filename);
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}
