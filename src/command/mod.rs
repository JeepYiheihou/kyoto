/* Command table to list all supported commands. */
pub mod command_table;

/* Command parser to convert bytes in buffer to commands.
 * This should be triggered in machine stage. */
pub mod command_parser;

/* Command executor to actually apply a command.
 * This should be triggered by machine stage and applied in warehouse stage. */
pub mod command_executor;