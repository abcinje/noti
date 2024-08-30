use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcInfo {
    pub command: String,
    pub pid: u32,
    pub status: String,
}