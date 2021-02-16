/* Entry of the whole collection of supported data structures.
 * This initialized at the start of server. */
pub mod db;

/* Command executor to actually apply a command.
 * This should be triggered by machine stage and applied in warehouse stage. */
 pub mod command_executor;