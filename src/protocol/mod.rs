/* Command table to list all supported commands. */
pub mod command;

/* Serialization (and deserialization) functionalities such as
 * converting bytes into commands and converting reponse into bytes.
 * This should be triggered in machine stage. */
pub mod serialization;

/* Expose Command enum. */
pub type Command = command::command_table::Command;

/* Expose CommandParser struct. */
pub type CommandParser = serialization::command_parser::CommandParser;

/* Encode response to bytes. */
pub type ResponseEncoder = serialization::response_encoder::ResponseEncoder;