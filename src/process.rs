use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Representa un proceso en el sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub pid: u32,
    pub state: ProcessState,
    pub cpu_burst: u64,         // Ráfaga de CPU total
    pub remaining_burst: u64,   // Ráfaga restante (para Round Robin)
    pub memory_req: usize,      // Memoria requerida
    pub arrival_time: u64,      // Tiempo de llegada
    pub start_time: Option<u64>, // Tiempo de primera ejecución
    pub finish_time: Option<u64>, // Tiempo de terminación
    pub io_operations: Vec<u64>, // Lista de operaciones I/O pendientes
}

impl Process {
    /// Crear un nuevo proceso
    pub fn new(pid: u32, cpu_burst: u64, memory_req: usize) -> Self {
        Self {
            pid,
            state: ProcessState::Ready,
            cpu_burst,
            remaining_burst: cpu_burst,
            memory_req,
            arrival_time: 0,
            start_time: None,
            finish_time: None,
            io_operations: Vec::new(),
        }
    }

    /// Crear proceso con tiempo de llegada específico
    pub fn with_arrival(pid: u32, cpu_burst: u64, memory_req: usize, arrival_time: u64) -> Self {
        Self {
            pid,
            state: ProcessState::Ready,
            cpu_burst,
            remaining_burst: cpu_burst,
            memory_req,
            arrival_time,
            start_time: None,
            finish_time: None,
            io_operations: Vec::new(),
        }
    }

    /// Calcular tiempo de espera
    pub fn waiting_time(&self, current_time: u64) -> u64 {
        if let Some(finish) = self.finish_time {
            finish.saturating_sub(self.arrival_time).saturating_sub(self.cpu_burst)
        } else {
            0
        }
    }

    /// Calcular tiempo de retorno (turnaround time)
    pub fn turnaround_time(&self) -> u64 {
        if let Some(finish) = self.finish_time {
            finish.saturating_sub(self.arrival_time)
        } else {
            0
        }
    }

    /// Calcular tiempo de respuesta
    pub fn response_time(&self) -> Option<u64> {
        self.start_time.map(|start| start.saturating_sub(self.arrival_time))
    }

    /// Marcar que el proceso ha comenzado a ejecutarse
    pub fn mark_started(&mut self, time: u64) {
        if self.start_time.is_none() {
            self.start_time = Some(time);
        }
    }

    /// Marcar que el proceso ha terminado
    pub fn mark_finished(&mut self, time: u64) {
        self.finish_time = Some(time);
        self.state = ProcessState::Terminated;
    }

    /// Verificar si el proceso ha terminado
    pub fn is_finished(&self) -> bool {
        self.remaining_burst == 0 || self.state == ProcessState::Terminated
    }
}