use crate::process::{Process};
use std::collections::VecDeque;

/// Trait para schedulers (permite intercambiar implementación)
pub trait Scheduler {
 fn push(&mut self, p: Process);
 fn next(&mut self) -> Option<Process>;
 fn is_empty(&self) -> bool;
 fn len(&self) -> usize;
 fn name(&self) -> &str;
}

/// Implementación FIFO simple
pub struct FifoScheduler {
 queue: VecDeque<Process>,
}

impl FifoScheduler {
 pub fn new() -> Self {
 Self { queue: VecDeque::new() }
 }
}

impl Scheduler for FifoScheduler {
 fn push(&mut self, p: Process) {
 self.queue.push_back(p);
 }

 fn next(&mut self) -> Option<Process> {
 self.queue.pop_front()
 }

 fn is_empty(&self) -> bool {
 self.queue.is_empty()
 }

 fn len(&self) -> usize {
 self.queue.len()
 }

 fn name(&self) -> &str {
 "FIFO"
 }
}

/// Implementación Round Robin con quantum fijo
pub struct RoundRobinScheduler {
 queue: VecDeque<Process>,
 quantum: u64,
}

impl RoundRobinScheduler {
 pub fn new(quantum: u64) -> Self {
 Self { 
 queue: VecDeque::new(),
 quantum,
 }
 }

 pub fn quantum(&self) -> u64 {
 self.quantum
 }
}

impl Scheduler for RoundRobinScheduler {
 fn push(&mut self, p: Process) {
 self.queue.push_back(p);
 }

 fn next(&mut self) -> Option<Process> {
 self.queue.pop_front()
 }

 fn is_empty(&self) -> bool {
 self.queue.is_empty()
 }

 fn len(&self) -> usize {
 self.queue.len()
 }

 fn name(&self) -> &str {
 "Round Robin"
 }
}

/// Implementación SJF (Shortest Job First) no expropiativo
pub struct SjfScheduler {
 processes: Vec<Process>,
}

impl SjfScheduler {
 pub fn new() -> Self {
 Self { processes: Vec::new() }
 }
}

impl Scheduler for SjfScheduler {
 fn push(&mut self, p: Process) {
 self.processes.push(p);
 // Mantener ordenado por remaining_burst (más corto primero)
 self.processes.sort_by_key(|p| p.remaining_burst);
 }

 fn next(&mut self) -> Option<Process> {
 if self.processes.is_empty() {
 None
 } else {
 Some(self.processes.remove(0))
 }
 }

 fn is_empty(&self) -> bool {
 self.processes.is_empty()
 }

 fn len(&self) -> usize {
 self.processes.len()
 }

 fn name(&self) -> &str {
 "SJF"
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn fifo_order() {
 let mut s = FifoScheduler::new();
 s.push(Process::new(1, 5, 10));
 s.push(Process::new(2, 3, 8));
 s.push(Process::new(3, 2, 4));

 assert_eq!(s.next().unwrap().pid, 1);
 assert_eq!(s.next().unwrap().pid, 2);
 assert_eq!(s.next().unwrap().pid, 3);
 }

 #[test]
 fn round_robin_fairness() {
 let mut rr = RoundRobinScheduler::new(3);
 
 let mut p1 = Process::new(1, 7, 10);
 let mut p2 = Process::new(2, 4, 8);
 let p3 = Process::new(3, 2, 4);

 rr.push(p1.clone());
 rr.push(p2.clone());
 rr.push(p3.clone());

 // Primera ronda
 let proc1 = rr.next().unwrap();
 assert_eq!(proc1.pid, 1);
 
 // Simular ejecución de quantum
 p1.remaining_burst = p1.remaining_burst.saturating_sub(3);
 if p1.remaining_burst > 0 {
 rr.push(p1.clone());
 }

 let proc2 = rr.next().unwrap();
 assert_eq!(proc2.pid, 2);
 
 p2.remaining_burst = p2.remaining_burst.saturating_sub(3);
 if p2.remaining_burst > 0 {
 rr.push(p2.clone());
 }

 let proc3 = rr.next().unwrap();
 assert_eq!(proc3.pid, 3);
 // P3 termina (2 < 3)

 // Segunda ronda - P1 y P2 restantes
 let proc1_again = rr.next().unwrap();
 assert_eq!(proc1_again.pid, 1);
 }

 #[test]
 fn sjf_shortest_first() {
 let mut sjf = SjfScheduler::new();
 
 sjf.push(Process::new(1, 7, 10));
 sjf.push(Process::new(2, 3, 8));
 sjf.push(Process::new(3, 5, 4));

 // Debe salir en orden: P2 (3), P3 (5), P1 (7)
 assert_eq!(sjf.next().unwrap().pid, 2);
 assert_eq!(sjf.next().unwrap().pid, 3);
 assert_eq!(sjf.next().unwrap().pid, 1);
 }
}