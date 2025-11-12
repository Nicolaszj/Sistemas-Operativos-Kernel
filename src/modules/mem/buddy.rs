/// Buddy System Allocator
/// Sistema de asignación de memoria con bloques de tamaño potencia de 2

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Estado de un bloque
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BlockState {
    Free,
    Allocated,
    Split,
}

/// Bloque de memoria en el Buddy System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub address: usize,
    pub size: usize,
    pub state: BlockState,
    pub pid: Option<u32>,
}

/// Buddy System Allocator
#[derive(Serialize, Deserialize)]
pub struct BuddyAllocator {
    total_size: usize,
    min_block_size: usize,
    blocks: Vec<Block>,
    allocated_blocks: HashMap<usize, Block>, // address -> block
    
    // Métricas
    total_allocations: u64,
    total_deallocations: u64,
    internal_fragmentation: usize,
}

impl BuddyAllocator {
    /// Crear un nuevo allocador
    /// total_size debe ser potencia de 2
    pub fn new(total_size: usize, min_block_size: usize) -> Self {
        // Crear bloque inicial libre
        let initial_block = Block {
            address: 0,
            size: total_size,
            state: BlockState::Free,
            pid: None,
        };

        Self {
            total_size,
            min_block_size,
            blocks: vec![initial_block],
            allocated_blocks: HashMap::new(),
            total_allocations: 0,
            total_deallocations: 0,
            internal_fragmentation: 0,
        }
    }

    /// Asignar memoria para un proceso
    pub fn alloc(&mut self, pid: u32, size: usize) -> Result<usize, String> {
        // Encontrar el tamaño de bloque más pequeño que pueda contener size
        let required_size = self.next_power_of_2(size.max(self.min_block_size));

        // Buscar un bloque libre del tamaño adecuado
        if let Some(block_idx) = self.find_free_block(required_size) {
            let block = self.blocks[block_idx].clone();
            
            // Si el bloque es más grande, dividirlo
            if block.size > required_size {
                self.split_block(block_idx, required_size);
            }

            // Marcar como asignado
            self.blocks[block_idx].state = BlockState::Allocated;
            self.blocks[block_idx].pid = Some(pid);
            
            let address = self.blocks[block_idx].address;
            self.allocated_blocks.insert(address, self.blocks[block_idx].clone());
            
            self.total_allocations += 1;
            self.internal_fragmentation += required_size - size;

            println!("[Buddy] Asignado {} bytes (redondeado a {}) en dirección {} para proceso {}", 
                size, required_size, address, pid);

            Ok(address)
        } else {
            Err(format!("No hay bloques libres suficientemente grandes para {} bytes", size))
        }
    }

    /// Liberar memoria
    pub fn free(&mut self, address: usize) -> Result<(), String> {
        if let Some(block) = self.allocated_blocks.remove(&address) {
            // Encontrar el bloque en la lista
            if let Some(idx) = self.blocks.iter().position(|b| b.address == address) {
                self.blocks[idx].state = BlockState::Free;
                self.blocks[idx].pid = None;
                
                // Intentar fusionar con buddies
                self.coalesce(idx);
                
                self.total_deallocations += 1;
                
                println!("[Buddy] Liberado bloque en dirección {} (tamaño: {})", address, block.size);
                
                Ok(())
            } else {
                Err(format!("Bloque en dirección {} no encontrado en lista", address))
            }
        } else {
            Err(format!("No hay bloque asignado en dirección {}", address))
        }
    }

    /// Encontrar un bloque libre del tamaño especificado
    fn find_free_block(&self, size: usize) -> Option<usize> {
        self.blocks.iter()
            .position(|b| b.state == BlockState::Free && b.size >= size)
    }

    /// Dividir un bloque en dos buddies
    fn split_block(&mut self, idx: usize, target_size: usize) {
        let block = self.blocks[idx].clone();
        
        if block.size <= target_size || block.size <= self.min_block_size * 2 {
            return;
        }

        let half_size = block.size / 2;
        
        // Marcar bloque original como dividido
        self.blocks[idx].state = BlockState::Split;
        self.blocks[idx].size = half_size;

        // Crear buddy
        let buddy = Block {
            address: block.address + half_size,
            size: half_size,
            state: BlockState::Free,
            pid: None,
        };

        self.blocks.insert(idx + 1, buddy);

        // Recursivamente dividir si es necesario
        if half_size > target_size {
            self.split_block(idx, target_size);
        } else {
            // Cambiar de Split a Free si ya es del tamaño correcto
            self.blocks[idx].state = BlockState::Free;
        }
    }

    /// Fusionar bloques buddy libres
    fn coalesce(&mut self, idx: usize) {
        if idx >= self.blocks.len() {
            return;
        }

        let block = self.blocks[idx].clone();
        
        // Buscar el buddy
        let buddy_address = self.get_buddy_address(block.address, block.size);
        
        if let Some(buddy_idx) = self.blocks.iter().position(|b| b.address == buddy_address && b.size == block.size) {
            if self.blocks[buddy_idx].state == BlockState::Free {
                // Ambos buddies están libres, fusionar
                let merged_address = block.address.min(buddy_address);
                let merged_size = block.size * 2;

                // Remover ambos bloques
                let remove_first = idx.min(buddy_idx);
                let remove_second = idx.max(buddy_idx);
                
                self.blocks.remove(remove_second);
                self.blocks.remove(remove_first);

                // Crear bloque fusionado
                let merged_block = Block {
                    address: merged_address,
                    size: merged_size,
                    state: BlockState::Free,
                    pid: None,
                };

                self.blocks.insert(remove_first, merged_block);

                // Recursivamente intentar fusionar más
                self.coalesce(remove_first);
            }
        }
    }

    /// Calcular dirección del buddy
    fn get_buddy_address(&self, address: usize, size: usize) -> usize {
        address ^ size
    }

    /// Siguiente potencia de 2
    fn next_power_of_2(&self, n: usize) -> usize {
        let mut power = self.min_block_size;
        while power < n {
            power *= 2;
        }
        power
    }

    /// Obtener métricas
    pub fn stats(&self) -> BuddyStats {
        let free_blocks = self.blocks.iter().filter(|b| b.state == BlockState::Free).count();
        let allocated_blocks = self.blocks.iter().filter(|b| b.state == BlockState::Allocated).count();
        let total_free = self.blocks.iter()
            .filter(|b| b.state == BlockState::Free)
            .map(|b| b.size)
            .sum();
        let total_allocated = self.blocks.iter()
            .filter(|b| b.state == BlockState::Allocated)
            .map(|b| b.size)
            .sum();

        BuddyStats {
            total_size: self.total_size,
            free_memory: total_free,
            allocated_memory: total_allocated,
            free_blocks,
            allocated_blocks,
            total_allocations: self.total_allocations,
            total_deallocations: self.total_deallocations,
            internal_fragmentation: self.internal_fragmentation,
            external_fragmentation: self.calculate_external_fragmentation(),
        }
    }

    /// Calcular fragmentación externa
    fn calculate_external_fragmentation(&self) -> f64 {
        let free_blocks: Vec<_> = self.blocks.iter()
            .filter(|b| b.state == BlockState::Free)
            .collect();
        
        if free_blocks.is_empty() {
            return 0.0;
        }

        let total_free: usize = free_blocks.iter().map(|b| b.size).sum();
        let largest_free = free_blocks.iter().map(|b| b.size).max().unwrap_or(0);
        
        if total_free == 0 {
            0.0
        } else {
            (1.0 - (largest_free as f64 / total_free as f64)) * 100.0
        }
    }

    /// Visualizar estado del allocador
    pub fn display(&self) {
        println!("\n╔════════════════════════════════════════════════════╗");
        println!("║         BUDDY ALLOCATOR - ESTADO ACTUAL            ║");
        println!("╠════════════════════════════════════════════════════╣");
        println!("║ Tamaño total: {} bytes                             ", self.total_size);
        println!("║ Tamaño mínimo de bloque: {} bytes                  ", self.min_block_size);
        println!("║ Bloques totales: {}                                ", self.blocks.len());
        println!("╠════════════════════════════════════════════════════╣");
        
        for block in &self.blocks {
            let state_str = match block.state {
                BlockState::Free => "Libre     ",
                BlockState::Allocated => "Asignado  ",
                BlockState::Split => "Dividido  ",
            };
            
            let pid_str = if let Some(pid) = block.pid {
                format!("PID {}", pid)
            } else {
                "-----".to_string()
            };
            
            println!("║ [{:#06x}] {} bytes | {} | {}              ", 
                block.address, block.size, state_str, pid_str);
        }
        
        println!("╚════════════════════════════════════════════════════╝");
    }
}

#[derive(Debug, Clone)]
pub struct BuddyStats {
    pub total_size: usize,
    pub free_memory: usize,
    pub allocated_memory: usize,
    pub free_blocks: usize,
    pub allocated_blocks: usize,
    pub total_allocations: u64,
    pub total_deallocations: u64,
    pub internal_fragmentation: usize,
    pub external_fragmentation: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buddy_alloc_free() {
        let mut allocator = BuddyAllocator::new(1024, 64);
        
        // Asignar 100 bytes (redondeado a 128)
        let addr1 = allocator.alloc(1, 100).unwrap();
        assert_eq!(addr1, 0);
        
        // Asignar 50 bytes (redondeado a 64)
        let addr2 = allocator.alloc(2, 50).unwrap();
        
        // Liberar primer bloque
        allocator.free(addr1).unwrap();
        
        let stats = allocator.stats();
        assert!(stats.allocated_memory > 0);
    }

    #[test]
    fn test_buddy_coalescing() {
        let mut allocator = BuddyAllocator::new(1024, 64);
        
        let addr1 = allocator.alloc(1, 64).unwrap();
        let addr2 = allocator.alloc(2, 64).unwrap();
        
        // Liberar ambos - deberían fusionarse
        allocator.free(addr1).unwrap();
        allocator.free(addr2).unwrap();
        
        let stats = allocator.stats();
        // Después de fusionar, debería haber menos bloques
        assert!(stats.free_blocks < 3);
    }
}
