use crate::process::Process;
use std::collections::VecDeque;

/// Trait para schedulers (permite intercambiar implementación)
pub trait Scheduler {
    fn push(&mut self, p: Process);
    fn next(&mut self) -> Option<Process>;
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
}