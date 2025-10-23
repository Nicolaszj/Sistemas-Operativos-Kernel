use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub pid: u32,
    pub state: ProcessState,
    pub burst: u64,
    pub memory_req: usize,
    // más campos como lista de operaciones I/O, tabla de páginas simulada, etc.
}

impl Process {
    pub fn new(pid: u32, burst: u64, memory_req: usize) -> Self {
        Self {
            pid,
            state: ProcessState::Ready,
            burst,
            memory_req,
        }
    }
}