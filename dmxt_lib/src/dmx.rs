// Paths
mod dmx_serial;
mod dmx_json;
mod types;
// Re-exports
pub mod serial { pub use super::dmx_serial::*; }
pub mod json { pub use super::dmx_json::*; }
pub use types::*;

