#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("GraphQL error: {0}")]
    GraphQLError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error(transparent)]
    KernelError(#[from] almond_kernel::error::KernelError),
}
