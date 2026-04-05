#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

/// Initialise the kernel (opens the database and runs migrations).
/// Call this once from Flutter before using any other API.
/// `database_url` examples:
///   - SQLite file : `"sqlite:///path/to/shurbs.db?mode=rwc"`
///   - In-memory   : `"sqlite::memory:"`
#[flutter_rust_bridge::frb]
pub async fn init_kernel(database_url: String) -> Result<(), String> {
    crate::state::init_kernel(database_url).await
}
