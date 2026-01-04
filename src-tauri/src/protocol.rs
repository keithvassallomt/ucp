use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Clipboard(String),
    PairRequest { msg: Vec<u8>, device_id: String },
    PairResponse { msg: Vec<u8>, device_id: String },
}
