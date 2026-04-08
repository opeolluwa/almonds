use crate::errors::app_error::AppError;

#[derive(Clone)]
pub struct RootService {}

impl RootService {
    pub fn init() -> Self {
        Self {}
    }
}
pub trait RootServiceTrait {
    fn health_check(&self) -> Result<(), AppError>;
}

impl RootServiceTrait for RootService {
    fn health_check(&self) -> Result<(), AppError> {
        log::info!("application is healthy ...");
        Ok(())
    }
}
