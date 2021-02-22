use crate::protocol::Command;
use crate::data::Server;

use bytes::{ Bytes, BytesMut };

pub struct CommandExecutor { }

impl CommandExecutor {

    /* Entry command to execute a command by its type. */
    pub fn execute_command(cmd: Command, server: &mut Server) -> crate::Result<Bytes> {
        match cmd {
            Command::Get { key } => {
                Self::_execute_get_cmd(server, key)
            },
            Command::Set { key, value } => {
                Self::_execute_set_cmd(server, key, value)
            },
            Command::Info {} => {
                Self::_execute_info_cmd(server)
            }
        }
    }

    /* Execute the GET command. */
    fn _execute_get_cmd(server: &mut Server,
                        key: String) -> crate::Result<Bytes> {
        match server.db.get(&key) {
            Some(res) => {
                Ok(res)
            },
            None => {
                Ok("Key not found.".into())
            }
        }
    }

    /* Execute the SET command. */
    fn _execute_set_cmd(server: &mut Server,
                        key: String,
                        value: Bytes)-> crate::Result<Bytes> {
        server.db.set(&key, value)?;
        Ok("OK.".into())
    }

    /* Execute the INFO command. */
    fn _execute_info_cmd(server: &mut Server) -> crate::Result<Bytes> {
        let mut info = BytesMut::from("");
        /* Get server config info. */
        {
            let server_config = server.server_config.lock().unwrap();
            info = server_config.generate_info(info);
        }

        /* Get machine info. */
        {
            let machine_info = server.machine_info.lock().unwrap();
            info = machine_info.generate_info(info);
        }

        /* Get data related info. */
        {
            let data_info = server.data_info.lock().unwrap();
            info = data_info.generate_info(info);
        }

        Ok(info.freeze())
    }
}