#![cfg(feature = "sqlite")]
use almond_kernel::{data_engine::Kernel, error::KernelError};

static DATABASE_PATH: &'static str = "sqlite://almond.dev.test.db?mode=rwc";

#[tokio::test]
async fn test_kernel_starts_wit_with_env() -> Result<(), KernelError> {
    let kernel = Kernel::new(DATABASE_PATH).await?;

    let migration_result = kernel.run_migrations().await;

    assert!(migration_result.is_ok());

    Ok(())
}
