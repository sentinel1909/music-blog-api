// src/lib/lib.rs

// module declarations
pub mod errors;
pub mod routes;
pub mod service;
pub mod telemetry;

// re-exports
pub use errors::*;
pub use service::*;
pub use telemetry::*;
