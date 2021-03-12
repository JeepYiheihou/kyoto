use bytes::Bytes;

#[derive(Debug)]
pub enum Command {
    /* Client initiated commands.
     * These commands are initiated by clients
     * which connect to the server.*/
    /* "GET" command. */
    Get {
        key: String,
    },
    /* "SET" command. */
    Set {
        key: String,
        value: Bytes,
    },
    /* "INFO" command. */
    Info { },
    /* "REPL_JOIN" command. */
    ReplJoin {
        addr: String,
        port: u16,
    },

    /* Node initiated commands.
     * These commands are initiated by other peer nodes
     * which connect to the server. */
    ReplPing { },

    /* Bad shaped command. */
    BadCommand {
        message: String,
    }
}