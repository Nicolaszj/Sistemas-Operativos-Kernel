use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

/// Semáforo básico (contador)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Semaphore {
 count: i32,
 waiting_queue: VecDeque<u32>, // PIDs esperando
 name: String,
}

impl Semaphore {
 pub fn new(initial_value: i32, name: &str) -> Self {
 Self {
 count: initial_value,
 waiting_queue: VecDeque::new(),
 name: name.to_string(),
 }
 }

 /// Operación wait (P) - Decrementar o bloquear
 pub fn wait(&mut self, pid: u32) -> bool {
 self.count -= 1;
 if self.count < 0 {
 // Bloquear proceso
 self.waiting_queue.push_back(pid);
 println!("[Semáforo {}] Proceso {} bloqueado (count={})", self.name, pid, self.count);
 false // Proceso bloqueado
 } else {
 println!("[Semáforo {}] Proceso {} pasó (count={})", self.name, pid, self.count);
 true // Proceso continúa
 }
 }

 /// Operación signal (V) - Incrementar y despertar
 pub fn signal(&mut self) -> Option<u32> {
 self.count += 1;
 if self.count <= 0 {
 // Despertar un proceso
 if let Some(pid) = self.waiting_queue.pop_front() {
 println!("[Semáforo {}] Proceso {} despertado (count={})", self.name, pid, self.count);
 return Some(pid);
 }
 }
 println!("[Semáforo {}] Signal ejecutado (count={})", self.name, self.count);
 None
 }

 pub fn count(&self) -> i32 {
 self.count
 }

 pub fn waiting_count(&self) -> usize {
 self.waiting_queue.len()
 }
}

/// Buffer compartido para productor-consumidor
#[derive(Debug, Serialize, Deserialize)]
pub struct ProducerConsumerBuffer {
 buffer: VecDeque<String>,
 capacity: usize,
 mutex: Semaphore,
 empty: Semaphore,
 full: Semaphore,
 total_produced: u64,
 total_consumed: u64,
}

impl ProducerConsumerBuffer {
 pub fn new(capacity: usize) -> Self {
 Self {
 buffer: VecDeque::with_capacity(capacity),
 capacity,
 mutex: Semaphore::new(1, "mutex"),
 empty: Semaphore::new(capacity as i32, "empty"),
 full: Semaphore::new(0, "full"),
 total_produced: 0,
 total_consumed: 0,
 }
 }

 /// Producir un item (bloquea si buffer lleno)
 pub fn produce(&mut self, item: String, producer_pid: u32) -> Result<(), String> {
 println!("\n[Productor {}] Intentando producir '{}'", producer_pid, item);
 
 // wait(empty) - ¿Hay espacio?
 if !self.empty.wait(producer_pid) {
 return Err(format!("Productor {} bloqueado: buffer lleno", producer_pid));
 }

 // wait(mutex) - Entrar a sección crítica
 if !self.mutex.wait(producer_pid) {
 return Err(format!("Productor {} bloqueado en mutex", producer_pid));
 }

 // SECCIÓN CRÍTICA: Agregar al buffer
 if self.buffer.len() >= self.capacity {
 self.mutex.signal();
 self.empty.signal();
 return Err("Buffer lleno (no debería pasar)".to_string());
 }

 self.buffer.push_back(item.clone());
 self.total_produced += 1;
 println!("[Productor {}] Item '{}' producido. Buffer: {}/{}", 
 producer_pid, item, self.buffer.len(), self.capacity);

 // signal(mutex) - Salir de sección crítica
 self.mutex.signal();

 // signal(full) - Indicar que hay un item disponible
 if let Some(consumer) = self.full.signal() {
 println!("[Productor {}] Despertó al consumidor {}", producer_pid, consumer);
 }

 Ok(())
 }

 /// Consumir un item (bloquea si buffer vacío)
 pub fn consume(&mut self, consumer_pid: u32) -> Result<String, String> {
 println!("\n[Consumidor {}] Intentando consumir", consumer_pid);

 // wait(full) - ¿Hay items?
 if !self.full.wait(consumer_pid) {
 return Err(format!("Consumidor {} bloqueado: buffer vacío", consumer_pid));
 }

 // wait(mutex) - Entrar a sección crítica
 if !self.mutex.wait(consumer_pid) {
 return Err(format!("Consumidor {} bloqueado en mutex", consumer_pid));
 }

 // SECCIÓN CRÍTICA: Extraer del buffer
 if let Some(item) = self.buffer.pop_front() {
 self.total_consumed += 1;
 println!("[Consumidor {}] Item '{}' consumido. Buffer: {}/{}", 
 consumer_pid, item, self.buffer.len(), self.capacity);

 // signal(mutex)
 self.mutex.signal();

 // signal(empty) - Indicar que hay espacio disponible
 if let Some(producer) = self.empty.signal() {
 println!("[Consumidor {}] Despertó al productor {}", consumer_pid, producer);
 }

 Ok(item)
 } else {
 self.mutex.signal();
 self.full.signal();
 Err("Buffer vacío (no debería pasar)".to_string())
 }
 }

 /// Mostrar estado del buffer
 pub fn status(&self) {
 println!("\n╔══════════════════════════════════════════════╗");
 println!("║ ESTADO DEL BUFFER PRODUCTOR-CONSUMIDOR ║");
 println!("╠══════════════════════════════════════════════╣");
 println!("║ Capacidad: {:2}/{:2} ║", self.buffer.len(), self.capacity);
 println!("║ Total producido: {:6} ║", self.total_produced);
 println!("║ Total consumido: {:6} ║", self.total_consumed);
 println!("╠══════════════════════════════════════════════╣");
 println!("║ Semáforos: ║");
 println!("║ mutex: count={:3} waiting={} ║", self.mutex.count(), self.mutex.waiting_count());
 println!("║ empty: count={:3} waiting={} ║", self.empty.count(), self.empty.waiting_count());
 println!("║ full: count={:3} waiting={} ║", self.full.count(), self.full.waiting_count());
 println!("╠══════════════════════════════════════════════╣");
 println!("║ Contenido del buffer: ║");
 
 if self.buffer.is_empty() {
 println!("║ (vacío) ║");
 } else {
 for (i, item) in self.buffer.iter().enumerate() {
 println!("║ [{:2}] {:36} ║", i, if item.len() > 36 { &item[..36] } else { item });
 }
 }
 
 println!("╚══════════════════════════════════════════════╝");
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_semaphore_basic() {
 let mut sem = Semaphore::new(1, "test");
 assert!(sem.wait(1)); // Debe pasar (1 -> 0)
 assert!(!sem.wait(2)); // Debe bloquear (0 -> -1)
 
 let unblocked = sem.signal(); // -1 -> 0, despierta PID 2
 assert_eq!(unblocked, Some(2));
 }

 #[test]
 fn test_producer_consumer() {
 let mut buffer = ProducerConsumerBuffer::new(2);
 
 // Producir 2 items (llenar buffer)
 assert!(buffer.produce("Item1".to_string(), 100).is_ok());
 assert!(buffer.produce("Item2".to_string(), 100).is_ok());
 
 // Consumir 1 item
 let item = buffer.consume(200).unwrap();
 assert_eq!(item, "Item1");
 
 // Producir otro (ahora hay espacio)
 assert!(buffer.produce("Item3".to_string(), 100).is_ok());
 
 assert_eq!(buffer.total_produced, 3);
 assert_eq!(buffer.total_consumed, 1);
 }
}
