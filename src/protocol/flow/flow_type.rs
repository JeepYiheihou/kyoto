use crate::protocol::{ Command, Response };

use bytes::BytesMut;

pub enum FlowType {

    /* From network stage to machine stage. */
    HandleSocketBuffer {
        buffer: BytesMut,
    },

    /* From machine stage to data stage. */
    ExecuteCommand {
        command: Command,
    },
}

pub enum RetFlowType {
    /* From machine stage to network stage. */
    SendResponse {
        response: Response,
    },

    /* From data stage to machine stage. */
    ReturnResponse {
        response: Response,
    },

    DoNothing { },
}