#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub args: Vec<String>,
    pub cwd: String,
}
