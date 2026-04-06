#[cfg(feature = "adapters")]
pub mod adapters;
pub mod entities;
pub mod error;
pub mod kernel;
#[cfg(feature = "pdf_generator")]
pub mod pdf_generator;
pub mod repositories;
#[cfg(feature = "sync_engine")]
pub mod sync_engine;
#[cfg(feature = "utils")]
pub mod utils;
// Core API
pub use kernel::DataEngine;

// Feature-gated re-exports
#[cfg(feature = "sync_engine")]
pub use graphql_ws_client::*;
pub use sea_orm;

pub use migration;
