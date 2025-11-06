use clap::{Parser, Subcommand};
use anyhow::Result;
use kernel_sim::kernel::{KernelState, SchedulerType};
use kernel_sim::modules::ipc::philosophers::DiningPhilosophers;
use kernel_sim::modules::disk::scheduler::{
    FcfsScheduler, SstfScheduler, ScanScheduler, ScanDirection, 
    DiskRequest, DiskSimulator, DiskScheduler
};

#[derive(Parser)]
#[command(name = "kernel-sim")]
#[command(about = "Simulación simplificada de núcleo (kernel) — CLI Completa", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inicializar el kernel con scheduler específico
    Init {
        #[arg(short, long, default_value = "rr")]
        scheduler: String, // rr, sjf, fifo
        
        #[arg(short, long, default_value = "4")]
        quantum: u64, // Solo para RR
        
        #[arg(short, long, default_value = "8")]
        frames: usize, // Número de marcos de memoria
    },

    /// Crear un nuevo proceso
    New {
        #[arg(short, long)]
        burst: u64,
        
        #[arg(short, long)]
        mem: usize,
    },

    /// Listar todos los procesos
    Ps,

    /// Avanzar n pasos de tiempo
    Tick {
        #[arg(default_value = "1")]
        steps: u64,
    },

    /// Terminar un proceso
    Kill {
        pid: u32,
    },

    /// Ejecutar n pasos completos
    Run {
        steps: u64,
    },

    /// Mostrar estado general del sistema
    Status,

    /// Calcular métricas de scheduling
    Metrics,

    /// Producir un item (productor-consumidor)
    Produce {
        item: String,
        
        #[arg(short, long, default_value = "100")]
        pid: u32,
    },

    /// Consumir un item (productor-consumidor)
    Consume {
        #[arg(short, long, default_value = "200")]
        pid: u32,
    },

    /// Mostrar estado del buffer productor-consumidor
    BufferStat,

    /// Simular acceso a memoria con FIFO
    MemFifo {
        pid: u32,
        pages: Vec<usize>, // Lista de páginas a acceder
    },

    /// Simular acceso a memoria con LRU
    MemLru {
        pid: u32,
        pages: Vec<usize>,
    },

    /// Mostrar marcos de memoria
    MemDisplay,

    /// Simular cena de los filósofos
    Philosophers {
        #[arg(short, long, default_value = "5")]
        count: usize,
        
        #[arg(short, long, default_value = "10")]
        steps: usize,
    },

    /// Simular planificación de disco FCFS
    DiskFcfs {
        #[arg(short, long, default_value = "50")]
        start: usize,
        
        cylinders: Vec<usize>,
    },

    /// Simular planificación de disco SSTF
    DiskSstf {
        #[arg(short, long, default_value = "50")]
        start: usize,
        
        cylinders: Vec<usize>,
    },

    /// Simular planificación de disco SCAN
    DiskScan {
        #[arg(short, long, default_value = "50")]
        start: usize,
        
        #[arg(short, long, default_value = "199")]
        max: usize,
        
        cylinders: Vec<usize>,
    },

    /// Comparar algoritmos de disco
    DiskCompare {
        #[arg(short, long, default_value = "50")]
        start: usize,
        
        #[arg(short, long, default_value = "199")]
        max: usize,
        
        cylinders: Vec<usize>,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    // Estado global (en aplicación real, esto se persistiría entre comandos)
    static mut KERNEL: Option<KernelState> = None;

    match cli.command {
        Commands::Init { scheduler, quantum, frames } => {
            let sched_type = match scheduler.as_str() {
                "rr" => SchedulerType::RoundRobin(quantum),
                "sjf" => SchedulerType::SJF,
                "fifo" => SchedulerType::FIFO,
                _ => {
                    println!("❌ Scheduler inválido. Usa: rr, sjf, o fifo");
                    return Ok(());
                }
            };

            unsafe {
                KERNEL = Some(KernelState::new(sched_type, frames));
                println!("✅ Kernel inicializado:");
                println!("   Scheduler: {}", scheduler);
                if scheduler == "rr" {
                    println!("   Quantum: {}", quantum);
                }
                println!("   Marcos de memoria: {}", frames);
            }
        }

        Commands::New { burst, mem } => {
            unsafe {
                if let Some(ref mut kernel) = KERNEL {
                    let pid = kernel.create_process(burst, mem);
                    println!("✅ Proceso {} creado (burst={}, mem={})", pid, burst, mem);
                } else {
                    println!("❌ Kernel no inicializado. Ejecuta: kernel-sim init");
                }
            }
        }

        Commands::Ps => {
            unsafe {
                if let Some(ref kernel) = KERNEL {
                    kernel.list_processes();
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::Tick { steps } => {
            unsafe {
                if let Some(ref mut kernel) = KERNEL {
                    kernel.tick(steps);
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::Kill { pid } => {
            unsafe {
                if let Some(ref mut kernel) = KERNEL {
                    match kernel.kill_process(pid) {
                        Ok(_) => println!("✅ Proceso {} terminado", pid),
                        Err(e) => println!("❌ Error: {}", e),
                    }
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::Run { steps } => {
            unsafe {
                if let Some(ref mut kernel) = KERNEL {
                    kernel.run(steps);
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::Status => {
            unsafe {
                if let Some(ref kernel) = KERNEL {
                    kernel.status();
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::Metrics => {
            unsafe {
                if let Some(ref kernel) = KERNEL {
                    kernel.compute_metrics();
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::Produce { item, pid } => {
            unsafe {
                if let Some(ref mut kernel) = KERNEL {
                    match kernel.produce(item.clone(), pid) {
                        Ok(_) => println!("✅ Item '{}' producido por proceso {}", item, pid),
                        Err(e) => println!("⚠️  {}", e),
                    }
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::Consume { pid } => {
            unsafe {
                if let Some(ref mut kernel) = KERNEL {
                    match kernel.consume(pid) {
                        Ok(item) => println!("✅ Proceso {} consumió: '{}'", pid, item),
                        Err(e) => println!("⚠️  {}", e),
                    }
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::BufferStat => {
            unsafe {
                if let Some(ref kernel) = KERNEL {
                    kernel.buffer_status();
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::MemFifo { pid, pages } => {
            unsafe {
                if let Some(ref mut kernel) = KERNEL {
                    println!("\n🔍 Simulando accesos con FIFO para proceso {}", pid);
                    for page in pages {
                        let _ = kernel.access_memory_fifo(pid, page);
                    }
                    kernel.display_memory();
                    let stats = kernel_sim::modules::mem::paging::FrameManager::new(8).stats();
                    println!("\nEstadísticas guardadas.");
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::MemLru { pid, pages } => {
            unsafe {
                if let Some(ref mut kernel) = KERNEL {
                    println!("\n🔍 Simulando accesos con LRU para proceso {}", pid);
                    for page in pages {
                        let _ = kernel.access_memory_lru(pid, page);
                    }
                    kernel.display_memory();
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::MemDisplay => {
            unsafe {
                if let Some(ref kernel) = KERNEL {
                    kernel.display_memory();
                } else {
                    println!("❌ Kernel no inicializado");
                }
            }
        }

        Commands::Philosophers { count, steps } => {
            println!("\n🍽️  Iniciando simulación: Cena de los Filósofos");
            let mut dining = DiningPhilosophers::new(count);
            dining.simulate(steps);
        }

        Commands::DiskFcfs { start, cylinders } => {
            println!("\n💾 Simulación de Disco - FCFS");
            let mut fcfs = FcfsScheduler::new();
            
            for (idx, cyl) in cylinders.iter().enumerate() {
                fcfs.add_request(DiskRequest {
                    pid: idx as u32 + 1,
                    cylinder: *cyl,
                    timestamp: idx as u64,
                });
            }

            let mut sim = DiskSimulator::new(start);
            sim.process_all(&mut fcfs);
            sim.visualize(199);
        }

        Commands::DiskSstf { start, cylinders } => {
            println!("\n💾 Simulación de Disco - SSTF");
            let mut sstf = SstfScheduler::new();
            
            for (idx, cyl) in cylinders.iter().enumerate() {
                sstf.add_request(DiskRequest {
                    pid: idx as u32 + 1,
                    cylinder: *cyl,
                    timestamp: idx as u64,
                });
            }

            let mut sim = DiskSimulator::new(start);
            sim.process_all(&mut sstf);
            sim.visualize(199);
        }

        Commands::DiskScan { start, max, cylinders } => {
            println!("\n💾 Simulación de Disco - SCAN");
            let mut scan = ScanScheduler::new(max, ScanDirection::Up);
            
            for (idx, cyl) in cylinders.iter().enumerate() {
                scan.add_request(DiskRequest {
                    pid: idx as u32 + 1,
                    cylinder: *cyl,
                    timestamp: idx as u64,
                });
            }

            let mut sim = DiskSimulator::new(start);
            sim.process_all(&mut scan);
            sim.visualize(max);
        }

        Commands::DiskCompare { start, max, cylinders } => {
            println!("\n💾 COMPARATIVA DE ALGORITMOS DE DISCO");
            println!("═══════════════════════════════════════\n");

            // FCFS
            let mut fcfs = FcfsScheduler::new();
            for (idx, cyl) in cylinders.iter().enumerate() {
                fcfs.add_request(DiskRequest {
                    pid: idx as u32 + 1,
                    cylinder: *cyl,
                    timestamp: idx as u64,
                });
            }
            let mut sim_fcfs = DiskSimulator::new(start);
            sim_fcfs.process_all(&mut fcfs);
            let fcfs_movement = sim_fcfs.total_movement();

            // SSTF
            let mut sstf = SstfScheduler::new();
            for (idx, cyl) in cylinders.iter().enumerate() {
                sstf.add_request(DiskRequest {
                    pid: idx as u32 + 1,
                    cylinder: *cyl,
                    timestamp: idx as u64,
                });
            }
            let mut sim_sstf = DiskSimulator::new(start);
            sim_sstf.process_all(&mut sstf);
            let sstf_movement = sim_sstf.total_movement();

            // SCAN
            let mut scan = ScanScheduler::new(max, ScanDirection::Up);
            for (idx, cyl) in cylinders.iter().enumerate() {
                scan.add_request(DiskRequest {
                    pid: idx as u32 + 1,
                    cylinder: *cyl,
                    timestamp: idx as u64,
                });
            }
            let mut sim_scan = DiskSimulator::new(start);
            sim_scan.process_all(&mut scan);
            let scan_movement = sim_scan.total_movement();

            // Resumen comparativo
            println!("\n╔════════════════════════════════════════════════╗");
            println!("║         RESUMEN COMPARATIVO                    ║");
            println!("╠════════════╦═══════════════╦══════════════════╣");
            println!("║ Algoritmo  ║   Movimiento  ║   Eficiencia     ║");
            println!("╠════════════╬═══════════════╬══════════════════╣");
            println!("║ FCFS       ║     {:4}      ║    Baseline      ║", fcfs_movement);
            println!("║ SSTF       ║     {:4}      ║    {:+.1}%        ║", 
                sstf_movement, 
                ((fcfs_movement as f64 - sstf_movement as f64) / fcfs_movement as f64) * 100.0);
            println!("║ SCAN       ║     {:4}      ║    {:+.1}%        ║", 
                scan_movement,
                ((fcfs_movement as f64 - scan_movement as f64) / fcfs_movement as f64) * 100.0);
            println!("╚════════════╩═══════════════╩══════════════════╝");

            let best = fcfs_movement.min(sstf_movement).min(scan_movement);
            let best_algo = if best == fcfs_movement {
                "FCFS"
            } else if best == sstf_movement {
                "SSTF"
            } else {
                "SCAN"
            };

            println!("\n🏆 Mejor algoritmo: {} (movimiento: {})", best_algo, best);
        }
    }

    Ok(())
}
