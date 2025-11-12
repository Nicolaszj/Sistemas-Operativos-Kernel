use anyhow::Result;
use log::info;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::process::{Process, ProcessState};
use crate::scheduler::{Scheduler, RoundRobinScheduler, SjfScheduler, FifoScheduler};
use crate::modules::mem::paging::FrameManager;
use crate::modules::mem::buddy::BuddyAllocator;
use crate::modules::ipc::sync::ProducerConsumerBuffer;

/// Estado global del kernel
#[derive(Serialize, Deserialize)]
pub struct KernelState {
 processes: HashMap<u32, Process>,
 next_pid: u32,
 
 #[serde(skip, default = "default_scheduler")]
 pub scheduler: Box<dyn Scheduler>,
 pub scheduler_type: SchedulerType,

 current_process: Option<u32>,
 memory_manager: FrameManager,
 heap_allocator: BuddyAllocator,
 producer_consumer: ProducerConsumerBuffer,
 current_time: u64,
 finished_processes: Vec<Process>,
}

/// Función para satisfacer el `default` de serde para el scheduler.
fn default_scheduler() -> Box<dyn Scheduler> {
 Box::new(FifoScheduler::new()) // Un valor por defecto temporal
}

impl KernelState {
 pub fn new(scheduler_type: SchedulerType, num_frames: usize) -> Self {
 Self {
 processes: HashMap::new(),
 next_pid: 1,
 scheduler: Self::create_scheduler(scheduler_type.clone()),
 scheduler_type,
 current_process: None,
 memory_manager: FrameManager::new(num_frames),
 heap_allocator: BuddyAllocator::new(4096, 64), // 4KB total, bloques mínimos de 64 bytes
 producer_consumer: ProducerConsumerBuffer::new(5),
 current_time: 0,
 finished_processes: Vec::new(),
 }
 }

 pub fn create_scheduler(scheduler_type: SchedulerType) -> Box<dyn Scheduler> {
 match scheduler_type {
 SchedulerType::RoundRobin(quantum) => Box::new(RoundRobinScheduler::new(quantum)),
 SchedulerType::SJF => Box::new(SjfScheduler::new()),
 SchedulerType::FIFO => Box::new(FifoScheduler::new()),
 }
 }

 /// Crear nuevo proceso
 pub fn create_process(&mut self, cpu_burst: u64, memory_req: usize) -> u32 {
 let pid = self.next_pid;
 self.next_pid += 1;

 let mut process = Process::with_arrival(pid, cpu_burst, memory_req, self.current_time);
 process.state = ProcessState::Ready;

 info!("Proceso {} creado (burst={}, mem={})", pid, cpu_burst, memory_req);
 
 // Crear tabla de páginas para el proceso
 self.memory_manager.create_page_table(pid);

 self.scheduler.push(process.clone());
 self.processes.insert(pid, process);

 pid
 }

 /// Devuelve una lista de procesos en estado Ready.
 pub fn get_ready_processes(&self) -> Vec<Process> {
 self.processes
 .values()
 .filter(|p| p.state == ProcessState::Ready)
 .cloned()
 .collect()
 }


 /// Listar procesos
 pub fn list_processes(&self) {
 println!("\n╔═══════════════════════════════════════════════════════════╗");
 println!("║ LISTA DE PROCESOS ║");
 println!("╠═════╦═══════════╦═══════════════╦═══════════╦═════════════╣");
 println!("║ PID ║ Estado ║ Burst Restante║ Memoria ║ Llegada ║");
 println!("╠═════╬═══════════╬═══════════════╬═══════════╬═════════════╣");

 let mut pids: Vec<_> = self.processes.keys().collect();
 pids.sort();

 for pid in pids {
 if let Some(proc) = self.processes.get(pid) {
 let state_str = format!("{:?}", proc.state);
 println!("║ {:3} ║ {:9} ║ {:4} ║ {:5} ║ {:3} ║",
 proc.pid,
 state_str,
 proc.remaining_burst,
 proc.memory_req,
 proc.arrival_time
 );
 }
 }

 if !self.finished_processes.is_empty() {
 println!("╠═════╩═══════════╩═══════════════╩═══════════╩═════════════╣");
 println!("║ PROCESOS TERMINADOS ║");
 println!("╠═════╦═══════════╦═══════════════╦═══════════╦═════════════╣");
 
 for proc in &self.finished_processes {
 println!("║ {:3} ║ Terminated║ 0 ║ {:5} ║ {:3} ║",
 proc.pid, proc.memory_req, proc.arrival_time);
 }
 }

 println!("╚═════╩═══════════╩═══════════════╩═══════════╩═════════════╝");
 println!("Scheduler activo: {} | Cola: {} procesos", 
 self.scheduler.name(), self.scheduler.len());
 }

 /// Terminar proceso
 pub fn kill_process(&mut self, pid: u32) -> Result<()> {
 if let Some(mut proc) = self.processes.remove(&pid) {
 proc.mark_finished(self.current_time);
 info!("Proceso {} terminado forzosamente", pid);
 self.finished_processes.push(proc);
 Ok(())
 } else {
 Err(anyhow::anyhow!("Proceso {} no encontrado", pid))
 }
 }

 /// Suspender proceso (bloquearlo)
 pub fn suspend_process(&mut self, pid: u32) -> Result<()> {
 if let Some(proc) = self.processes.get_mut(&pid) {
 if proc.state == ProcessState::Running || proc.state == ProcessState::Ready {
 proc.state = ProcessState::Blocked;
 info!("Proceso {} suspendido", pid);
 Ok(())
 } else {
 Err(anyhow::anyhow!("Proceso {} no puede ser suspendido (estado: {:?})", pid, proc.state))
 }
 } else {
 Err(anyhow::anyhow!("Proceso {} no encontrado", pid))
 }
 }

 /// Reanudar proceso (desbloquearlo)
 pub fn resume_process(&mut self, pid: u32) -> Result<()> {
 if let Some(proc) = self.processes.get_mut(&pid) {
 if proc.state == ProcessState::Blocked {
 proc.state = ProcessState::Ready;
 // Volver a agregarlo al scheduler
 self.scheduler.push(proc.clone());
 info!("Proceso {} reanudado", pid);
 Ok(())
 } else {
 Err(anyhow::anyhow!("Proceso {} no está bloqueado (estado: {:?})", pid, proc.state))
 }
 } else {
 Err(anyhow::anyhow!("Proceso {} no encontrado", pid))
 }
 }

 /// Avanzar n pasos de simulación
 pub fn tick(&mut self, steps: u64) {
 println!("\n╔═════════════════════════════════════════╗");
 println!("║ Avanzando {} paso(s) de simulación ", steps);
 println!("╚═════════════════════════════════════════╝");

 for step in 0..steps {
 self.current_time += 1;
 println!("\n[TIME] Tiempo: {} (paso {})", self.current_time, step + 1);

 // Obtener siguiente proceso del scheduler
 if let Some(mut process) = self.scheduler.next() {
 println!("-> CPU: Proceso {} ejecutando...", process.pid);
 
 // Marcar como iniciado si es primera vez
 process.mark_started(self.current_time);
 process.state = ProcessState::Running;

 // Ejecutar por 1 unidad de tiempo
 let quantum_used = 1.min(process.remaining_burst);
 process.remaining_burst -= quantum_used;

 println!(" Ejecutó {} unidad(es), restante: {}", quantum_used, process.remaining_burst);

 // Verificar si terminó
 if process.remaining_burst == 0 {
 process.mark_finished(self.current_time);
 println!("[OK] Proceso {} TERMINADO", process.pid);
 // Eliminar de la lista de procesos activos y añadir la versión actualizada a los finalizados
 if self.processes.remove(&process.pid).is_some() {
 self.finished_processes.push(process);
 }
 } else {
 // Volver a la cola
 process.state = ProcessState::Ready;
 self.processes.insert(process.pid, process.clone());
 self.scheduler.push(process.clone());
 }
 } else {
 println!(" (CPU inactiva - no hay procesos)");
 }
 }

 println!("\n[TIME] Tiempo actual: {}", self.current_time);
 }

 /// Ejecutar n pasos completos
 pub fn run(&mut self, steps: u64) {
 self.tick(steps);
 }

 /// Mostrar estado general
 pub fn status(&self) {
 println!("\n╔═══════════════════════════════════════════════════════════╗");
 println!("║ ESTADO GENERAL DEL SISTEMA ║");
 println!("╠═══════════════════════════════════════════════════════════╣");
 println!("║ Tiempo actual: {:6} ║", self.current_time);
 println!("║ Scheduler: {:20} ║", self.scheduler.name());
 println!("║ Procesos activos: {:3} ║", self.processes.len());
 println!("║ Procesos en cola: {:3} ║", self.scheduler.len());
 println!("║ Procesos finalizados: {:3} ║", self.finished_processes.len());
 println!("╚═══════════════════════════════════════════════════════════╝");

 // Métricas de memoria
 let mem_stats = self.memory_manager.stats();
 println!("\n╔═══════════════════════════════════════════════════════════╗");
 println!("║ MÉTRICAS DE MEMORIA ║");
 println!("╠═══════════════════════════════════════════════════════════╣");
 println!("║ Fallos de página: {:6} ║", mem_stats.page_faults);
 println!("║ Aciertos (hits): {:6} ║", mem_stats.page_hits);
 println!("║ Tasa de aciertos: {:.2}% ║", mem_stats.hit_rate);
 println!("║ Accesos totales: {:6} ║", mem_stats.total_accesses);
 println!("╚═══════════════════════════════════════════════════════════╝");

 // Mostrar marcos de memoria
 self.memory_manager.display_frames();
 }

 /// Calcular métricas finales
 pub fn compute_metrics(&self) {
 if self.finished_processes.is_empty() {
 println!("\nNo hay procesos terminados para calcular métricas.");
 return;
 }

 let mut total_waiting = 0u64;
 let mut total_turnaround = 0u64;
 let mut total_response = 0u64;
 let mut response_count = 0;

 println!("\n╔═══════════════════════════════════════════════════════════════════╗");
 println!("║ MÉTRICAS DE SCHEDULING ║");
 println!("╠═════╦═══════════╦═══════════════╦═══════════════╦═════════════════╣");
 println!("║ PID ║ Llegada ║ Tiempo Espera ║ Turnaround ║ Respuesta ║");
 println!("╠═════╬═══════════╬═══════════════╬═══════════════╬═════════════════╣");

 for proc in &self.finished_processes {
 let waiting = proc.waiting_time(self.current_time);
 let turnaround = proc.turnaround_time();
 let response = proc.response_time();

 total_waiting += waiting;
 total_turnaround += turnaround;
 
 if let Some(resp) = response {
 total_response += resp;
 response_count += 1;
 }

 let resp_str = response.map(|r| format!("{:6}", r)).unwrap_or_else(|| " N/A ".to_string());
 
 println!("║ {:3} ║ {:3} ║ {:4} ║ {:4} ║ {} ║",
 proc.pid, proc.arrival_time, waiting, turnaround, resp_str);
 }

 println!("╠═════╩═══════════╩═══════════════╩═══════════════╩═════════════════╣");

 let count = self.finished_processes.len() as f64;
 let avg_waiting = total_waiting as f64 / count;
 let avg_turnaround = total_turnaround as f64 / count;
 let avg_response = if response_count > 0 {
 total_response as f64 / response_count as f64
 } else {
 0.0
 };

 println!("║ PROMEDIOS: ║");
 println!("║ Tiempo de espera: {:.2} ", avg_waiting);
 println!("║ Tiempo de retorno: {:.2} ", avg_turnaround);
 println!("║ Tiempo de respuesta: {:.2} ", avg_response);
 println!("╚═══════════════════════════════════════════════════════════════════╝");
 }

 /// Acceder a gestión de productor-consumidor
 pub fn produce(&mut self, item: String, producer_pid: u32) -> Result<()> {
 self.producer_consumer.produce(item, producer_pid)
 .map_err(|e| anyhow::anyhow!(e))
 }

 pub fn consume(&mut self, consumer_pid: u32) -> Result<String> {
 self.producer_consumer.consume(consumer_pid)
 .map_err(|e| anyhow::anyhow!(e))
 }

 pub fn buffer_status(&self) {
 self.producer_consumer.status();
 }

 /// Acceder a marcos de memoria
 pub fn display_memory(&self) {
 self.memory_manager.display_frames();
 }

 /// Simular acceso a memoria con algoritmo FIFO
 pub fn access_memory_fifo(&mut self, pid: u32, page: usize) -> Result<()> {
 match self.memory_manager.access_page_fifo(pid, page) {
 Ok(frame) => {
 println!("[OK] Acceso a página {} del proceso {} -> Marco {}", page, pid, frame);
 Ok(())
 }
 Err(e) => Err(anyhow::anyhow!(e))
 }
 }

 /// Simular acceso a memoria con algoritmo LRU
 pub fn access_memory_lru(&mut self, pid: u32, page: usize) -> Result<()> {
 match self.memory_manager.access_page_lru(pid, page) {
 Ok(frame) => {
 println!("[OK] Acceso a página {} del proceso {} -> Marco {}", page, pid, frame);
 Ok(())
 }
 Err(e) => Err(anyhow::anyhow!(e))
 }
 }

 /// Simular acceso a memoria con algoritmo Working Set
 pub fn access_memory_ws(&mut self, pid: u32, page: usize, window: usize) -> Result<()> {
 match self.memory_manager.access_page_working_set(pid, page, window) {
 Ok(frame) => {
 println!("[OK] Acceso a página {} del proceso {} -> Marco {} (WS ventana={})", page, pid, frame, window);
 Ok(())
 }
 Err(e) => Err(anyhow::anyhow!(e))
 }
 }

 /// Asignar memoria heap con Buddy Allocator
 pub fn heap_alloc(&mut self, pid: u32, size: usize) -> Result<usize> {
 self.heap_allocator.alloc(pid, size)
 .map_err(|e| anyhow::anyhow!(e))
 }

 /// Liberar memoria heap
 pub fn heap_free(&mut self, address: usize) -> Result<()> {
 self.heap_allocator.free(address)
 .map_err(|e| anyhow::anyhow!(e))
 }

 /// Mostrar estado del heap allocator
 pub fn heap_status(&self) {
 self.heap_allocator.display();
 
 let stats = self.heap_allocator.stats();
 println!("\n╔════════════════════════════════════════════════════╗");
 println!("║ ESTADÍSTICAS DEL HEAP ALLOCATOR ║");
 println!("╠════════════════════════════════════════════════════╣");
 println!("║ Memoria libre: {} bytes ", stats.free_memory);
 println!("║ Memoria asignada: {} bytes ", stats.allocated_memory);
 println!("║ Bloques libres: {} ", stats.free_blocks);
 println!("║ Bloques asignados: {} ", stats.allocated_blocks);
 println!("║ Total asignaciones: {} ", stats.total_allocations);
 println!("║ Total liberaciones: {} ", stats.total_deallocations);
 println!("║ Fragmentación interna: {} bytes ", stats.internal_fragmentation);
 println!("║ Fragmentación externa: {:.2}% ", stats.external_fragmentation);
 println!("╚════════════════════════════════════════════════════╝");
 }
}

/// Tipos de scheduler disponibles
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SchedulerType {
 RoundRobin(u64),
 SJF,
 FIFO,
}

/// Función principal de arranque
pub fn run(config: Option<String>) -> Result<()> {
 info!("kernel-sim: arrancando (config: {:?})", config);
 println!("Kernel simulado iniciado. Usa la CLI para interactuar.");
 Ok(())
}
