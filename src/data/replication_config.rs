use std::vec::Vec;
use std::net::SocketAddr;

#[derive(Debug)]
pub enum ReplicationRole {
    Primary,
    Replica,
    Confused,
}

#[derive(Debug)]
pub struct ReplicationNode {
    addr: SocketAddr,
}

#[derive(Debug)]
pub struct ReplicationConfig {
    role: ReplicationRole,
    replicas: Vec<ReplicationNode>
}

impl ReplicationNode {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr: addr,
        }
    }
}

impl ReplicationConfig {
    pub fn new() -> Self {
        let role = ReplicationRole::Confused;
        let replicas = Vec::new();
        Self {
            role: role,
            replicas: replicas,
        }
    }

    pub fn add_replica_node(&mut self, addr: SocketAddr) {
        let new_node = ReplicationNode::new(addr);
        self.replicas.push(new_node);
    }
}