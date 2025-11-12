//! Memory manager with paging support

pub mod paging;
pub mod buddy;

pub struct MemoryManager {}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {}
    }

    pub fn alloc(&mut self, pid: u32, size: usize) -> Result<usize, &'static str> {
        println!("(mem) alloc pid={} size={}", pid, size);
        Ok(0) // dirección simulada
    }

    pub fn free(&mut self, pid: u32) {
        println!("(mem) free pid={}", pid);
    }
}
