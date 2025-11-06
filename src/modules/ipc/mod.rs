//! IPC & Sync - Semáforos y sincronización

pub mod sync;
pub mod philosophers;

use std::collections::HashMap;

pub struct IpcRegistry {
    sems: HashMap<String, sync::Semaphore>,
}

impl IpcRegistry {
    pub fn new() -> Self {
        Self { sems: HashMap::new() }
    }

    pub fn create_semaphore(&mut self, name: String, initial_value: i32) {
        self.sems.insert(name.clone(), sync::Semaphore::new(initial_value, &name));
    }

    pub fn get_semaphore(&mut self, name: &str) -> Option<&mut sync::Semaphore> {
        self.sems.get_mut(name)
    }
}
