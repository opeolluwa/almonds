pub mod api;
pub mod commands;
pub mod error;
pub mod state;

mod frb_generated;

// Re-export kernel init so Flutter can call it on app start.
pub use state::init_kernel;
