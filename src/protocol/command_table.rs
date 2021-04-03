use bytes::Bytes;

#[derive(Debug)]
pub enum Command {
    /* Client initiated commands.
     * These commands are initiated by clients
     * which connect to the server.*/
    /* "GET" command. */
    Get {
        key: String,
        id: i64,
    },
    /* "SET" command. */
    Set {
        key: String,
        value: Bytes,
        id: i64,
    },
    /* "INFO" command. */
    Info {
        id: i64,
    },
    /* "REPL_JOIN" command. */
    ReplJoin {
        addr: String,
        port: u16,
        id: i64,
    },

    /* Node initiated commands.
     * These commands are initiated by other peer nodes
     * which connect to the server. */
    ReplPing {
        id: i64,
    },

    /* Bad shaped command. */
    BadCommand {
        message: String,
    }
}