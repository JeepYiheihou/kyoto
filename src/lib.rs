/* Network module.
 * This is the module to handle network IO. */
pub mod network;

/* Machine module.
 * This is the module to handle machine operations.
 * e.g. command parsing */
pub mod machine;

/* Data module.
 * This is the module for data structures.
 * Commands are actually executed here. Memory IO and disk IO. */
pub mod data;

/* Protocol module.
 * This is the module for command protocol utils.
 * Command table, command parser, etc. This is not a module for any
 * concept "stage", but used in stages. */
pub mod protocol;

pub mod benchmark;

/* kyoto Error type. */
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/* Result type for kyoto operations.
 * This is defined as a convenience. */
pub type Result<T> = std::result::Result<T, Error>;