use super::sync::Semaphore;

/// Estado de un filósofo
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhilosopherState {
    Thinking,
    Hungry,
    Eating,
    Finished,
}

/// Un filósofo
#[derive(Debug)]
pub struct Philosopher {
    pub id: usize,
    pub state: PhilosopherState,
    pub times_eaten: u32,
}

/// Simulación de la Cena de los Filósofos
pub struct DiningPhilosophers {
    philosophers: Vec<Philosopher>,
    forks: Vec<Semaphore>, // Un semáforo por tenedor
    total_philosophers: usize,
}

impl DiningPhilosophers {
    pub fn new(count: usize) -> Self {
        let mut philosophers = Vec::with_capacity(count);
        let mut forks = Vec::with_capacity(count);

        for i in 0..count {
            philosophers.push(Philosopher {
                id: i,
                state: PhilosopherState::Thinking,
                times_eaten: 0,
            });
            forks.push(Semaphore::new(1, &format!("fork_{}", i)));
        }

        Self {
            philosophers,
            forks,
            total_philosophers: count,
        }
    }

    /// Un filósofo intenta comer
    /// Solución: Orden asimétrico para evitar deadlock
    /// - Filósofos 0-(n-2): toman tenedor izquierdo, luego derecho
    /// - Filósofo (n-1): toma tenedor derecho, luego izquierdo
    pub fn try_eat(&mut self, philosopher_id: usize) -> Result<(), String> {
        if philosopher_id >= self.total_philosophers {
            return Err("Filósofo inválido".to_string());
        }

        let left_fork = philosopher_id;
        let right_fork = (philosopher_id + 1) % self.total_philosophers;

        println!("\n[Filósofo {}] Tengo hambre...", philosopher_id);
        self.philosophers[philosopher_id].state = PhilosopherState::Hungry;

        // Orden asimétrico para evitar deadlock
        let (first_fork, second_fork) = if philosopher_id == self.total_philosophers - 1 {
            // Último filósofo: derecho primero
            (right_fork, left_fork)
        } else {
            // Otros: izquierdo primero
            (left_fork, right_fork)
        };

        // Intentar tomar primer tenedor
        if !self.forks[first_fork].wait(philosopher_id as u32) {
            println!("[Filósofo {}] No pude tomar tenedor {}", philosopher_id, first_fork);
            return Err(format!("Filósofo {} bloqueado en tenedor {}", philosopher_id, first_fork));
        }

        println!("[Filósofo {}] Tomé tenedor {}", philosopher_id, first_fork);

        // Intentar tomar segundo tenedor
        if !self.forks[second_fork].wait(philosopher_id as u32) {
            // Si no puede tomar el segundo, suelta el primero
            println!("[Filósofo {}] No pude tomar tenedor {}, soltando {}", 
                philosopher_id, second_fork, first_fork);
            self.forks[first_fork].signal();
            return Err(format!("Filósofo {} bloqueado en tenedor {}", philosopher_id, second_fork));
        }

        println!("[Filósofo {}] Tomé tenedor {}", philosopher_id, second_fork);

        // ¡A comer!
        self.philosophers[philosopher_id].state = PhilosopherState::Eating;
        self.philosophers[philosopher_id].times_eaten += 1;
        
        println!("🍝 [Filósofo {}] COMIENDO (vez #{}) con tenedores {} y {}", 
            philosopher_id, self.philosophers[philosopher_id].times_eaten, 
            left_fork, right_fork);

        // Simular tiempo comiendo (en sistema real sería sleep)
        // En nuestra simulación, come instantáneamente

        // Soltar tenedores en orden inverso
        self.forks[second_fork].signal();
        println!("[Filósofo {}] Solté tenedor {}", philosopher_id, second_fork);
        
        self.forks[first_fork].signal();
        println!("[Filósofo {}] Solté tenedor {}", philosopher_id, first_fork);

        // Volver a pensar
        self.philosophers[philosopher_id].state = PhilosopherState::Thinking;
        println!("💭 [Filósofo {}] Pensando...", philosopher_id);

        Ok(())
    }

    /// Simular varios pasos
    pub fn simulate(&mut self, steps: usize) {
        println!("\n╔═══════════════════════════════════════════════════╗");
        println!("║       SIMULACIÓN: CENA DE LOS FILÓSOFOS          ║");
        println!("║  {} filósofos, {} tenedores                         ║", 
            self.total_philosophers, self.total_philosophers);
        println!("╚═══════════════════════════════════════════════════╝");

        for step in 0..steps {
            println!("\n--- Paso {} ---", step + 1);
            
            // Cada filósofo intenta comer en secuencia
            for i in 0..self.total_philosophers {
                let _ = self.try_eat(i);
            }

            self.display_status();
        }

        self.display_summary();
    }

    /// Mostrar estado actual
    pub fn display_status(&self) {
        println!("\n┌─────┬───────────┬────────────┐");
        println!("│ ID  │  Estado   │ Veces Comió│");
        println!("├─────┼───────────┼────────────┤");
        
        for phil in &self.philosophers {
            let state_str = match phil.state {
                PhilosopherState::Thinking => "💭 Pensando",
                PhilosopherState::Hungry => "😋 Hambriento",
                PhilosopherState::Eating => "🍝 Comiendo",
                PhilosopherState::Finished => "✅ Terminó",
            };
            println!("│  {}  │ {} │     {:3}    │", phil.id, state_str, phil.times_eaten);
        }
        
        println!("└─────┴───────────┴────────────┘");
    }

    /// Resumen final
    pub fn display_summary(&self) {
        println!("\n╔═══════════════════════════════════════════╗");
        println!("║          RESUMEN FINAL                    ║");
        println!("╠═══════════════════════════════════════════╣");
        
        let total_meals: u32 = self.philosophers.iter().map(|p| p.times_eaten).sum();
        let avg_meals = total_meals as f64 / self.total_philosophers as f64;
        
        println!("║ Total de comidas: {:6}                 ║", total_meals);
        println!("║ Promedio por filósofo: {:.2}             ║", avg_meals);
        println!("╠═══════════════════════════════════════════╣");
        
        let min_meals = self.philosophers.iter().map(|p| p.times_eaten).min().unwrap_or(0);
        let max_meals = self.philosophers.iter().map(|p| p.times_eaten).max().unwrap_or(0);
        
        println!("║ Mínimo: {} | Máximo: {}                    ║", min_meals, max_meals);
        
        if min_meals > 0 {
            println!("║ ✅ NO hubo inanición                     ║");
        } else {
            println!("║ ⚠️  Posible inanición detectada          ║");
        }
        
        println!("╚═══════════════════════════════════════════╝");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_philosophers_no_deadlock() {
        let mut dining = DiningPhilosophers::new(5);
        
        // Simular varios intentos
        for _ in 0..10 {
            for i in 0..5 {
                let _ = dining.try_eat(i);
            }
        }

        // Verificar que todos comieron al menos una vez (no hay inanición)
        for phil in &dining.philosophers {
            assert!(phil.times_eaten > 0, "Filósofo {} no comió", phil.id);
        }
    }
}
