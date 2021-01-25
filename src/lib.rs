pub mod network;

pub mod data_structure;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
/* Result type for osaka operations.
 * This is defined as a convinience */
pub type Result<T> = std::result::Result<T, Error>;