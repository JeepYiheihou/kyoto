pub mod protocol;

pub mod data;

pub mod network;

/* kyoto Error type. */
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/* Result type for kyoto operations.
 * This is defined as a convenience. */
pub type Result<T> = std::result::Result<T, Error>;