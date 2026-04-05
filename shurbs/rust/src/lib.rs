mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod error;
pub mod state;
pub mod api;



// Re-export kernel init so Flutter can call it on app start.
pub use state::init_kernel;
