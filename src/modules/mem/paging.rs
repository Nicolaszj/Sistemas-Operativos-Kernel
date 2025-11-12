use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Entrada en la tabla de páginas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTableEntry {
    pub frame_number: Option<usize>, // None = no está en memoria
    pub valid: bool,
    pub last_access: u64, // Timestamp para LRU
}

/// Tabla de páginas por proceso
#[derive(Debug, Serialize, Deserialize)]
pub struct PageTable {
    pub pid: u32,
    entries: HashMap<usize, PageTableEntry>, // page_number -> entry
}

impl PageTable {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            entries: HashMap::new(),
        }
    }

    /// Acceder a una página
    pub fn access(&mut self, page_num: usize, time: u64) -> Option<usize> {
        if let Some(entry) = self.entries.get_mut(&page_num) {
            entry.last_access = time;
            if entry.valid {
                return entry.frame_number;
            }
        } else {
            // Primera vez que se accede a esta página
            self.entries.insert(page_num, PageTableEntry {
                frame_number: None,
                valid: false,
                last_access: time,
            });
        }
        None
    }

    /// Mapear página a marco
    pub fn map(&mut self, page_num: usize, frame_num: usize, time: u64) {
        self.entries.insert(page_num, PageTableEntry {
            frame_number: Some(frame_num),
            valid: true,
            last_access: time,
        });
    }

    /// Invalidar entrada (cuando se reemplaza)
    pub fn invalidate(&mut self, page_num: usize) {
        if let Some(entry) = self.entries.get_mut(&page_num) {
            entry.valid = false;
            entry.frame_number = None;
        }
    }

    /// Obtener todas las páginas válidas
    pub fn valid_pages(&self) -> Vec<(usize, usize, u64)> {
        self.entries
            .iter()
            .filter(|(_, e)| e.valid)
            .map(|(page, e)| (*page, e.frame_number.unwrap(), e.last_access))
            .collect()
    }
}

/// Marco de memoria física
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub frame_num: usize,
    pub pid: Option<u32>,
    pub page_num: Option<usize>,
    pub load_time: u64, // Para FIFO
}

/// Gestor de marcos de memoria con algoritmos de reemplazo
#[derive(Serialize, Deserialize)]
pub struct FrameManager {
    frames: Vec<Frame>,
    fifo_queue: VecDeque<usize>, // Índices de marcos para FIFO
    page_tables: HashMap<u32, PageTable>,
    
    // Métricas
    page_faults: u64,
    page_hits: u64,
    current_time: u64,
}

impl FrameManager {
    pub fn new(total_frames: usize) -> Self {
        let mut frames = Vec::with_capacity(total_frames);
        for i in 0..total_frames {
            frames.push(Frame {
                frame_num: i,
                pid: None,
                page_num: None,
                load_time: 0,
            });
        }

        Self {
            frames,
            fifo_queue: VecDeque::new(),
            page_tables: HashMap::new(),
            page_faults: 0,
            page_hits: 0,
            current_time: 0,
        }
    }

    /// Crear tabla de páginas para un proceso
    pub fn create_page_table(&mut self, pid: u32) {
        self.page_tables.insert(pid, PageTable::new(pid));
    }

    /// Acceder a una página (devuelve marco o provoca fallo)
    pub fn access_page_fifo(&mut self, pid: u32, page_num: usize) -> Result<usize, String> {
        self.current_time += 1;

        // Verificar si la página tabla existe
        if !self.page_tables.contains_key(&pid) {
            self.create_page_table(pid);
        }

        // Intentar acceder a la página
        let page_table = self.page_tables.get_mut(&pid).unwrap();
        
        if let Some(frame_num) = page_table.access(page_num, self.current_time) {
            // HIT: La página ya está en memoria
            self.page_hits += 1;
            return Ok(frame_num);
        }

        // MISS: Fallo de página
        self.page_faults += 1;

        // Buscar marco libre
        if let Some(free_idx) = self.frames.iter().position(|f| f.pid.is_none()) {
            // Hay marco libre
            self.allocate_frame_fifo(free_idx, pid, page_num);
            return Ok(free_idx);
        }

        // No hay marcos libres - reemplazar con FIFO
        let victim_idx = self.fifo_queue.pop_front().unwrap();
        self.replace_frame_fifo(victim_idx, pid, page_num);
        Ok(victim_idx)
    }

    /// Acceder a una página con LRU
    pub fn access_page_lru(&mut self, pid: u32, page_num: usize) -> Result<usize, String> {
        self.current_time += 1;

        if !self.page_tables.contains_key(&pid) {
            self.create_page_table(pid);
        }

        let page_table = self.page_tables.get_mut(&pid).unwrap();
        
        if let Some(frame_num) = page_table.access(page_num, self.current_time) {
            // HIT
            self.page_hits += 1;
            return Ok(frame_num);
        }

        // MISS
        self.page_faults += 1;

        // Buscar marco libre
        if let Some(free_idx) = self.frames.iter().position(|f| f.pid.is_none()) {
            self.allocate_frame_lru(free_idx, pid, page_num);
            return Ok(free_idx);
        }

        // Reemplazar con LRU
        let victim_idx = self.find_lru_victim();
        self.replace_frame_lru(victim_idx, pid, page_num);
        Ok(victim_idx)
    }

    /// Asignar marco libre (FIFO)
    fn allocate_frame_fifo(&mut self, frame_idx: usize, pid: u32, page_num: usize) {
        self.frames[frame_idx] = Frame {
            frame_num: frame_idx,
            pid: Some(pid),
            page_num: Some(page_num),
            load_time: self.current_time,
        };
        self.fifo_queue.push_back(frame_idx);
        
        let page_table = self.page_tables.get_mut(&pid).unwrap();
        page_table.map(page_num, frame_idx, self.current_time);
    }

    /// Reemplazar marco (FIFO)
    fn replace_frame_fifo(&mut self, frame_idx: usize, new_pid: u32, new_page: usize) {
        // Invalidar entrada antigua
        if let Some(old_pid) = self.frames[frame_idx].pid {
            if let Some(old_page) = self.frames[frame_idx].page_num {
                if let Some(old_table) = self.page_tables.get_mut(&old_pid) {
                    old_table.invalidate(old_page);
                }
            }
        }

        // Asignar nuevo
        self.frames[frame_idx] = Frame {
            frame_num: frame_idx,
            pid: Some(new_pid),
            page_num: Some(new_page),
            load_time: self.current_time,
        };
        self.fifo_queue.push_back(frame_idx);

        let page_table = self.page_tables.get_mut(&new_pid).unwrap();
        page_table.map(new_page, frame_idx, self.current_time);
    }

    /// Asignar marco libre (LRU)
    fn allocate_frame_lru(&mut self, frame_idx: usize, pid: u32, page_num: usize) {
        self.frames[frame_idx] = Frame {
            frame_num: frame_idx,
            pid: Some(pid),
            page_num: Some(page_num),
            load_time: self.current_time,
        };
        
        let page_table = self.page_tables.get_mut(&pid).unwrap();
        page_table.map(page_num, frame_idx, self.current_time);
    }

    /// Encontrar víctima LRU
    fn find_lru_victim(&self) -> usize {
        let mut min_time = u64::MAX;
        let mut victim_idx = 0;

        for (idx, frame) in self.frames.iter().enumerate() {
            if let Some(pid) = frame.pid {
                if let Some(page_num) = frame.page_num {
                    if let Some(table) = self.page_tables.get(&pid) {
                        if let Some(entry) = table.entries.get(&page_num) {
                            if entry.last_access < min_time {
                                min_time = entry.last_access;
                                victim_idx = idx;
                            }
                        }
                    }
                }
            }
        }

        victim_idx
    }

    /// Reemplazar marco (LRU)
    fn replace_frame_lru(&mut self, frame_idx: usize, new_pid: u32, new_page: usize) {
        // Invalidar entrada antigua
        if let Some(old_pid) = self.frames[frame_idx].pid {
            if let Some(old_page) = self.frames[frame_idx].page_num {
                if let Some(old_table) = self.page_tables.get_mut(&old_pid) {
                    old_table.invalidate(old_page);
                }
            }
        }

        // Asignar nuevo
        self.frames[frame_idx] = Frame {
            frame_num: frame_idx,
            pid: Some(new_pid),
            page_num: Some(new_page),
            load_time: self.current_time,
        };

        let page_table = self.page_tables.get_mut(&new_pid).unwrap();
        page_table.map(new_page, frame_idx, self.current_time);
    }

    /// Obtener métricas
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            page_faults: self.page_faults,
            page_hits: self.page_hits,
            hit_rate: if self.page_hits + self.page_faults > 0 {
                (self.page_hits as f64 / (self.page_hits + self.page_faults) as f64) * 100.0
            } else {
                0.0
            },
            total_accesses: self.page_hits + self.page_faults,
        }
    }

    /// Visualizar marcos
    pub fn display_frames(&self) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║         ESTADO DE MARCOS DE MEMORIA        ║");
        println!("╠═══════╦══════╦═══════╦══════════════════════╣");
        println!("║ Marco ║  PID ║ Pág # ║    Cargado (time)    ║");
        println!("╠═══════╬══════╬═══════╬══════════════════════╣");
        
        for frame in &self.frames {
            let pid_str = frame.pid.map(|p| format!("{:4}", p)).unwrap_or_else(|| " -- ".to_string());
            let page_str = frame.page_num.map(|p| format!("{:5}", p)).unwrap_or_else(|| "  -- ".to_string());
            let time_str = if frame.pid.is_some() {
                format!("{:20}", frame.load_time)
            } else {
                "       (libre)       ".to_string()
            };
            
            println!("║  {:3}  ║ {} ║ {} ║ {} ║", 
                frame.frame_num, pid_str, page_str, time_str);
        }
        
        println!("╚═══════╩══════╩═══════╩══════════════════════╝");
    }

    /// Resetear métricas
    pub fn reset_stats(&mut self) {
        self.page_faults = 0;
        self.page_hits = 0;
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub page_faults: u64,
    pub page_hits: u64,
    pub hit_rate: f64,
    pub total_accesses: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifo_replacement() {
        let mut fm = FrameManager::new(3);
        fm.create_page_table(1);

        // Secuencia de accesos que causa reemplazo
        let pages = vec![1, 2, 3, 4, 1, 2, 5];
        
        for page in pages {
            let _ = fm.access_page_fifo(1, page);
        }

        let stats = fm.stats();
        assert!(stats.page_faults > 0);
        assert!(stats.page_hits > 0);
    }

    #[test]
    fn test_lru_replacement() {
        let mut fm = FrameManager::new(3);
        fm.create_page_table(1);

        let pages = vec![1, 2, 3, 4, 1, 2, 5];
        
        for page in pages {
            let _ = fm.access_page_lru(1, page);
        }

        let stats = fm.stats();
        assert!(stats.page_faults > 0);
    }
}
