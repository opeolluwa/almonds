use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Kernel(#[from] almond_kernel::error::KernelError),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Path error: {0}")]
    Path(String),
}

impl AppError {
    pub fn io(e: impl std::fmt::Display) -> Self {
        AppError::Io(e.to_string())
    }

    pub fn path(e: impl std::fmt::Display) -> Self {
        AppError::Path(e.to_string())
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
