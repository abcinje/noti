use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProcInfo {
    pub command: String,
    pub pid: u32,
    pub is_term: bool,
    pub status: String,
}