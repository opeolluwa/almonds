pub struct AppLogger {}

impl AppLogger {
    pub fn init() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_target(false)
            .compact()
            .init();
    }
}
