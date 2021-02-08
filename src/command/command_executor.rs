use bytes::Bytes;

use crate::command::command_table::Command;
use crate::warehouse::db::Db;

pub struct CommandExecutor { }

impl CommandExecutor {
    pub fn execute_command(cmd: Command, db: &mut Db) -> crate::Result<Bytes> {
        match cmd {
            Command::Get { key } => {
                match db.get(&key) {
                    Some(res) => {
                        Ok(res)
                    },
                    None => {
                        Ok("Key not found.".into())
                    }
                }
            },
            Command::Set { key, value } => {
                db.set(&key, value)?;
                Ok("OK.".into())
            }
        }
    }
}