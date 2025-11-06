use std::collections::VecDeque;

/// Solicitud de acceso a disco
#[derive(Debug, Clone)]
pub struct DiskRequest {
    pub pid: u32,
    pub cylinder: usize,
    pub timestamp: u64,
}

/// Trait para algoritmos de planificación de disco
pub trait DiskScheduler {
    fn add_request(&mut self, request: DiskRequest);
    fn next_request(&mut self, current_position: usize) -> Option<DiskRequest>;
    fn is_empty(&self) -> bool;
    fn name(&self) -> &str;
}

/// FCFS (First Come First Served) - Orden de llegada
pub struct FcfsScheduler {
    queue: VecDeque<DiskRequest>,
}

impl FcfsScheduler {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl DiskScheduler for FcfsScheduler {
    fn add_request(&mut self, request: DiskRequest) {
        self.queue.push_back(request);
    }

    fn next_request(&mut self, _current_position: usize) -> Option<DiskRequest> {
        self.queue.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn name(&self) -> &str {
        "FCFS"
    }
}

/// SSTF (Shortest Seek Time First) - Más cercano primero
pub struct SstfScheduler {
    requests: Vec<DiskRequest>,
}

impl SstfScheduler {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }
}

impl DiskScheduler for SstfScheduler {
    fn add_request(&mut self, request: DiskRequest) {
        self.requests.push(request);
    }

    fn next_request(&mut self, current_position: usize) -> Option<DiskRequest> {
        if self.requests.is_empty() {
            return None;
        }

        // Encontrar la solicitud más cercana
        let mut min_distance = usize::MAX;
        let mut closest_idx = 0;

        for (idx, req) in self.requests.iter().enumerate() {
            let distance = if req.cylinder > current_position {
                req.cylinder - current_position
            } else {
                current_position - req.cylinder
            };

            if distance < min_distance {
                min_distance = distance;
                closest_idx = idx;
            }
        }

        Some(self.requests.remove(closest_idx))
    }

    fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    fn name(&self) -> &str {
        "SSTF"
    }
}

/// SCAN (Algoritmo del Ascensor) - Barre en una dirección
pub struct ScanScheduler {
    requests: Vec<DiskRequest>,
    direction: ScanDirection, // 1 = hacia arriba, -1 = hacia abajo
    max_cylinder: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScanDirection {
    Up,
    Down,
}

impl ScanScheduler {
    pub fn new(max_cylinder: usize, initial_direction: ScanDirection) -> Self {
        Self {
            requests: Vec::new(),
            direction: initial_direction,
            max_cylinder,
        }
    }
}

impl DiskScheduler for ScanScheduler {
    fn add_request(&mut self, request: DiskRequest) {
        self.requests.push(request);
    }

    fn next_request(&mut self, current_position: usize) -> Option<DiskRequest> {
        if self.requests.is_empty() {
            return None;
        }

        // Ordenar solicitudes por cilindro
        self.requests.sort_by_key(|r| r.cylinder);

        match self.direction {
            ScanDirection::Up => {
                // Buscar la siguiente solicitud hacia arriba
                if let Some(idx) = self.requests.iter().position(|r| r.cylinder >= current_position) {
                    return Some(self.requests.remove(idx));
                }
                // Si no hay hacia arriba, cambiar dirección
                self.direction = ScanDirection::Down;
                self.requests.pop()
            }
            ScanDirection::Down => {
                // Buscar la siguiente solicitud hacia abajo
                if let Some(idx) = self.requests.iter().rposition(|r| r.cylinder <= current_position) {
                    return Some(self.requests.remove(idx));
                }
                // Si no hay hacia abajo, cambiar dirección
                self.direction = ScanDirection::Up;
                self.requests.first().map(|_| self.requests.remove(0))
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    fn name(&self) -> &str {
        "SCAN"
    }
}

/// Simulador de disco con métricas
pub struct DiskSimulator {
    current_position: usize,
    total_movement: usize,
    requests_served: usize,
    history: Vec<(usize, usize)>, // (from, to) para visualización
}

impl DiskSimulator {
    pub fn new(initial_position: usize) -> Self {
        Self {
            current_position: initial_position,
            total_movement: 0,
            requests_served: 0,
            history: Vec::new(),
        }
    }

    /// Procesar todas las solicitudes con un scheduler
    pub fn process_all<S: DiskScheduler>(&mut self, scheduler: &mut S) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║  SIMULACIÓN DE DISCO - {}                  ", scheduler.name());
        println!("╠════════════════════════════════════════════╣");
        println!("║ Posición inicial: {:3}                     ║", self.current_position);
        println!("╚════════════════════════════════════════════╝");

        while !scheduler.is_empty() {
            if let Some(request) = scheduler.next_request(self.current_position) {
                self.serve_request(request);
            }
        }

        self.display_summary();
    }

    /// Servir una solicitud
    fn serve_request(&mut self, request: DiskRequest) {
        let movement = if request.cylinder > self.current_position {
            request.cylinder - self.current_position
        } else {
            self.current_position - request.cylinder
        };

        println!("  Cabezal: {:3} → {:3}  (movimiento: {:3})  [PID {}]",
            self.current_position, request.cylinder, movement, request.pid);

        self.history.push((self.current_position, request.cylinder));
        self.total_movement += movement;
        self.current_position = request.cylinder;
        self.requests_served += 1;
    }

    /// Mostrar resumen
    pub fn display_summary(&self) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║              RESUMEN                       ║");
        println!("╠════════════════════════════════════════════╣");
        println!("║ Solicitudes atendidas: {:4}               ║", self.requests_served);
        println!("║ Movimiento total:      {:4} cilindros     ║", self.total_movement);
        
        if self.requests_served > 0 {
            let avg_movement = self.total_movement as f64 / self.requests_served as f64;
            println!("║ Promedio por solicitud: {:.2} cilindros   ║", avg_movement);
        }
        
        println!("║ Posición final:        {:3}                ║", self.current_position);
        println!("╚════════════════════════════════════════════╝");
    }

    /// Visualizar recorrido
    pub fn visualize(&self, max_cylinder: usize) {
        println!("\n┌─ Visualización del recorrido (0-{}) ─┐", max_cylinder);
        
        for (from, to) in &self.history {
            let min = (*from).min(*to);
            let max = (*from).max(*to);
            
            print!("│ ");
            for cyl in 0..=max_cylinder {
                if cyl == *from {
                    print!("O");
                } else if cyl == *to {
                    print!("X");
                } else if cyl > min && cyl < max {
                    print!("─");
                } else {
                    print!(" ");
                }
            }
            println!(" │ {} → {}", from, to);
        }
        
        println!("└" + &"─".repeat(max_cylinder + 4) + "┘");
    }

    pub fn total_movement(&self) -> usize {
        self.total_movement
    }

    pub fn reset(&mut self, initial_position: usize) {
        self.current_position = initial_position;
        self.total_movement = 0;
        self.requests_served = 0;
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fcfs_order() {
        let mut fcfs = FcfsScheduler::new();
        
        fcfs.add_request(DiskRequest { pid: 1, cylinder: 98, timestamp: 1 });
        fcfs.add_request(DiskRequest { pid: 2, cylinder: 183, timestamp: 2 });
        fcfs.add_request(DiskRequest { pid: 3, cylinder: 37, timestamp: 3 });

        assert_eq!(fcfs.next_request(50).unwrap().cylinder, 98);
        assert_eq!(fcfs.next_request(98).unwrap().cylinder, 183);
        assert_eq!(fcfs.next_request(183).unwrap().cylinder, 37);
    }

    #[test]
    fn test_sstf_closest_first() {
        let mut sstf = SstfScheduler::new();
        
        sstf.add_request(DiskRequest { pid: 1, cylinder: 98, timestamp: 1 });
        sstf.add_request(DiskRequest { pid: 2, cylinder: 183, timestamp: 2 });
        sstf.add_request(DiskRequest { pid: 3, cylinder: 37, timestamp: 3 });

        // Desde posición 50, el más cercano es 37 (distancia 13)
        assert_eq!(sstf.next_request(50).unwrap().cylinder, 37);
    }

    #[test]
    fn test_disk_simulator() {
        let mut fcfs = FcfsScheduler::new();
        fcfs.add_request(DiskRequest { pid: 1, cylinder: 10, timestamp: 1 });
        fcfs.add_request(DiskRequest { pid: 2, cylinder: 30, timestamp: 2 });
        
        let mut sim = DiskSimulator::new(0);
        sim.process_all(&mut fcfs);
        
        // Movimiento: 0->10 (10) + 10->30 (20) = 30
        assert_eq!(sim.total_movement(), 30);
    }
}
