/* Command module. */
pub mod command;

/* Response type. */
pub mod response;

/* Serialization (and deserialization) functionalities such as
 * converting bytes into commands and converting reponse into bytes.
 * This should be triggered in machine stage. */
pub mod serialization;

/* FlowType and RetFlowType. */
pub mod flow;

/* Expose Command enum. */
pub type Command = command::command_table::Command;

/* Expose CommandParser struct. */
pub type CommandParser = serialization::command_parser::CommandParser;

/* Expose Response struct. */
pub type Response = response::response::Response;

/* Expose ResponseEncoder struct. */
pub type ResponseEncoder = serialization::response_encoder::ResponseEncoder;

/* Expose FlowType. */
pub type FlowType = flow::flow_type::FlowType;

/* Expose RetFlowType. */
pub type RetFlowType = flow::flow_type::RetFlowType;