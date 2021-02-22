use crate::data::server_state::server_config::ServerConfig;
use crate::data::server_state::machine_info::MachineInfo;
use crate::data::warehouse::data_info::DataInfo;
use crate::data::warehouse::db::Db;
use crate::protocol::{ Command, Response, FlowType, RetFlowType };

use bytes::{ Bytes, BytesMut };
use std::sync::{ Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Server {
    pub server_config: Arc<Mutex<ServerConfig>>,
    pub machine_info: Arc<Mutex<MachineInfo>>,
    pub data_info: Arc<Mutex<DataInfo>>,
    pub db: Arc<Db>,
}

/* The middle layer between Server and ServerState/Db
 * so that it can be wrapped with Arc and then making server able to
 * be cloned and provided to multiple threads.
 * 
 * This is only for Arc, not for Mutex. Locks need to be handled separately. */
impl Server {
    pub fn new() -> Self {
        let server_config = Arc::new(Mutex::new(ServerConfig::new()));
        let machine_info = Arc::new(Mutex::new(MachineInfo::new()));
        let data_info = Arc::new(Mutex::new(DataInfo::new()));
        let db = Arc::new(Db::new());
        Self {
            server_config: server_config,
            machine_info: machine_info,
            data_info: data_info,
            db: db,
        }
    }

    pub fn handle_flow(&mut self, flow: FlowType) -> crate::Result<RetFlowType> {
        match flow {
            FlowType::ExecuteCommand{ command } => {
                self.execute_command(command)
            }
            _ => {
                Err("Invalid flow.".into())
            }
        }
    }

    /* Entry command to execute a command by its type. */
    pub fn execute_command(&mut self, cmd: Command) -> crate::Result<RetFlowType> {
        match cmd {
            Command::Get { key } => {
                self._execute_get_cmd(key)
            },
            Command::Set { key, value } => {
                self._execute_set_cmd(key, value)
            },
            Command::Info {} => {
                self._execute_info_cmd()
            }
        }
    }

    /* Execute the GET command. */
    fn _execute_get_cmd(&mut self,
                        key: String) -> crate::Result<RetFlowType> {
        match self.db.get(&key) {
            Some(res) => {
                let response = Response::Valid{ message: res };
                let ret_flow = RetFlowType::ReturnResponse{ response: response };
                Ok(ret_flow)
            },
            None => {
                let response = Response::Valid{ message: "Key not found.".into() };
                let ret_flow = RetFlowType::ReturnResponse{ response: response };
                Ok(ret_flow)
            }
        }
    }

    /* Execute the SET command. */
    fn _execute_set_cmd(&mut self,
                        key: String,
                        value: Bytes)-> crate::Result<RetFlowType> {
        self.db.set(&key, value)?;
        let response = Response::Valid{ message: "Ok.".into() };
        let ret_flow = RetFlowType::ReturnResponse{ response: response };
        Ok(ret_flow)
    }

    /* Execute the INFO command. */
    fn _execute_info_cmd(&mut self) -> crate::Result<RetFlowType> {
        let mut info = BytesMut::from("");
        /* Get server config info. */
        {
            let server_config = self.server_config.lock().unwrap();
            info = server_config.generate_info(info);
        }

        /* Get machine info. */
        {
            let machine_info = self.machine_info.lock().unwrap();
            info = machine_info.generate_info(info);
        }

        /* Get data related info. */
        {
            let data_info = self.data_info.lock().unwrap();
            info = data_info.generate_info(info);
        }

        let response = Response::Valid{ message: info.freeze() };
        let ret_flow = RetFlowType::ReturnResponse{ response: response };
        Ok(ret_flow)
    }
}