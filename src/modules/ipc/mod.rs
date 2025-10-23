//! IPC & Sync skeleton — semáforos básicos (simulados)

use std::collections::HashMap;

pub struct Semaphore {
    value: i32,
}

impl Semaphore {
    pub fn new(initial: i32) -> Self {
        Self { value: initial }
    }

    pub fn wait(&mut self) {
        // placeholder
        if self.value > 0 {
            self.value -= 1;
        }
    }

    pub fn post(&mut self) {
        self.value += 1;
    }
}

pub struct IpcRegistry {
    sems: HashMap<String, Semaphore>,
}

impl IpcRegistry {
    pub fn new() -> Self {
        Self { sems: HashMap::new() }
    }
}