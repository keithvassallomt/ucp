use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub ip: IpAddr,
    pub port: u16,
    pub hostname: String,
    pub last_seen: u64, // timestamp for pruning old peers
}
