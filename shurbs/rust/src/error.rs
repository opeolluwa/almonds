use almond_kernel::error::KernelError;

/// FRB-compatible error type. All errors collapse to a String so Dart can
/// receive them as a plain exception message.
#[derive(Debug)]
pub enum AppError {
    Kernel(KernelError),
    Io(String),
    Parse(String),
}

impl From<KernelError> for AppError {
    fn from(e: KernelError) -> Self {
        AppError::Kernel(e)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Kernel(e) => write!(f, "{e}"),
            AppError::Io(e) => write!(f, "IO error: {e}"),
            AppError::Parse(e) => write!(f, "Parse error: {e}"),
        }
    }
}

impl From<AppError> for String {
    fn from(e: AppError) -> Self {
        e.to_string()
    }
}

/// Parse a UUID string — returns AppError::Parse on failure.
pub fn parse_uuid(s: &str) -> Result<uuid::Uuid, AppError> {
    uuid::Uuid::parse_str(s).map_err(|e| AppError::Parse(format!("invalid UUID '{s}': {e}")))
}

/// Build an Option<RequestMeta> from an optional workspace-id string.
pub fn make_meta(
    workspace_id: Option<String>,
) -> Result<Option<almond_kernel::adapters::meta::RequestMeta>, AppError> {
    match workspace_id {
        None => Ok(None),
        Some(id) => Ok(Some(almond_kernel::adapters::meta::RequestMeta {
            workspace_identifier: parse_uuid(&id)?,
        })),
    }
}
