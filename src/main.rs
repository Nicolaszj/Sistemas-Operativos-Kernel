use clap::{Parser, Subcommand};
use anyhow::Result;
use kernel_sim::kernel::{KernelState, SchedulerType};
use std::fs;
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
    /// Borrar el estado del kernel y empezar de nuevo
    Reset,

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

    /// Suspender un proceso (bloquearlo)
    Suspend {
        pid: u32,
    },

    /// Reanudar un proceso (desbloquearlo)
    Resume {
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
        #[arg(long)]
        pid: u32,
        pages: Vec<usize>, // Lista de páginas a acceder
    },

    /// Simular acceso a memoria con LRU
    MemLru {
        #[arg(long)]
        pid: u32,
        pages: Vec<usize>,
    },

    /// Simular acceso a memoria con Working Set
    MemWs {
        #[arg(long)]
        pid: u32,
        
        #[arg(short, long, default_value = "10")]
        window: usize,
        
        pages: Vec<usize>,
    },

    /// Mostrar marcos de memoria
    MemDisplay,

    /// Asignar memoria heap (Buddy Allocator)
    HeapAlloc {
        #[arg(short, long)]
        pid: u32,
        
        size: usize,
    },

    /// Liberar memoria heap
    HeapFree {
        address: usize,
    },

    /// Mostrar estado del heap allocator
    HeapStatus,

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
        
        #[arg(long, value_delimiter = ' ', num_args = 1..)]
        cylinders: Vec<usize>,
    },
}

const KERNEL_STATE_FILE: &str = "kernel_state.json";

/// Carga el estado del kernel desde un archivo JSON.
fn load_kernel() -> Result<Option<KernelState>> {
    if let Ok(data) = fs::read_to_string(KERNEL_STATE_FILE) {
        // Deserializa el estado
        let mut kernel: KernelState = serde_json::from_str(&data)?;
        
        // 1. Reconstruye el scheduler, que no se guarda directamente
        let scheduler_type = kernel.scheduler_type.clone();
        kernel.scheduler = KernelState::create_scheduler(scheduler_type);

        // 2. Repuebla el scheduler con los procesos que están en estado 'Ready'
        for process in kernel.get_ready_processes() {
            kernel.scheduler.push(process);
        }

        Ok(Some(kernel))
    } else {
        Ok(None)
    }
}

/// Guarda el estado del kernel en un archivo JSON.
fn save_kernel(kernel: &KernelState) -> Result<()> {
    let data = serde_json::to_string_pretty(kernel)?;
    fs::write(KERNEL_STATE_FILE, data)?;
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Reset => {
            if fs::remove_file(KERNEL_STATE_FILE).is_ok() {
                println!("✅ Estado del kernel reseteado. Puedes iniciar uno nuevo con `init`.");
            } else {
                println!("ℹ️ No había un estado del kernel para resetear.");
            }
            return Ok(()); // Salir después de resetear
        }

        Commands::Init { scheduler, quantum, frames } => {
            let sched_type = match scheduler.as_str() {
                "rr" => SchedulerType::RoundRobin(quantum),
                "sjf" => SchedulerType::SJF,
                "fifo" => SchedulerType::FIFO,
                _ => {
                    eprintln!("❌ Scheduler inválido. Usa: rr, sjf, o fifo");
                    return Ok(());
                }
            };
            let kernel = KernelState::new(sched_type, frames);
            save_kernel(&kernel)?;
            println!("✅ Kernel inicializado:");
            println!("   Scheduler: {}", scheduler);
            if scheduler == "rr" {
                println!("   Quantum: {}", quantum);
            }
            println!("   Marcos de memoria: {}", frames);
        }

        Commands::New { burst, mem } => {
            if let Some(mut kernel) = load_kernel()? {
                let pid = kernel.create_process(burst, mem);
                println!("✅ Proceso {} creado (burst={}, mem={})", pid, burst, mem);
                save_kernel(&kernel)?;
            } else {
                eprintln!("❌ Kernel no inicializado. Ejecuta: kernel-sim init");
            }
        }

        Commands::Ps => {
            if let Some(kernel) = load_kernel()? {
                kernel.list_processes();
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::Tick { steps } => {
            if let Some(mut kernel) = load_kernel()? {
                kernel.tick(steps);
                save_kernel(&kernel)?;
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::Kill { pid } => {
            if let Some(mut kernel) = load_kernel()? {
                match kernel.kill_process(pid) {
                    Ok(_) => {
                        println!("✅ Proceso {} terminado", pid);
                        save_kernel(&kernel)?;
                    }
                    Err(e) => eprintln!("❌ Error: {}", e),
                }
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::Suspend { pid } => {
            if let Some(mut kernel) = load_kernel()? {
                match kernel.suspend_process(pid) {
                    Ok(_) => {
                        println!("⏸️  Proceso {} suspendido (bloqueado)", pid);
                        save_kernel(&kernel)?;
                    }
                    Err(e) => eprintln!("❌ Error: {}", e),
                }
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::Resume { pid } => {
            if let Some(mut kernel) = load_kernel()? {
                match kernel.resume_process(pid) {
                    Ok(_) => {
                        println!("▶️  Proceso {} reanudado", pid);
                        save_kernel(&kernel)?;
                    }
                    Err(e) => eprintln!("❌ Error: {}", e),
                }
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::Run { steps } => {
            if let Some(mut kernel) = load_kernel()? {
                kernel.run(steps);
                save_kernel(&kernel)?;
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::Status => {
            if let Some(kernel) = load_kernel()? {
                kernel.status();
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::Metrics => {
            if let Some(kernel) = load_kernel()? {
                kernel.compute_metrics();
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::Produce { item, pid } => {
            if let Some(mut kernel) = load_kernel()? {
                match kernel.produce(item.clone(), pid) {
                    Ok(_) => {
                        println!("✅ Item '{}' producido por proceso {}", item, pid);
                        save_kernel(&kernel)?;
                    }
                    Err(e) => eprintln!("⚠️  {}", e),
                }
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::Consume { pid } => {
            if let Some(mut kernel) = load_kernel()? {
                match kernel.consume(pid) {
                    Ok(item) => {
                        println!("✅ Proceso {} consumió: '{}'", pid, item);
                        save_kernel(&kernel)?;
                    }
                    Err(e) => eprintln!("⚠️  {}", e),
                }
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::BufferStat => {
            if let Some(kernel) = load_kernel()? {
                kernel.buffer_status();
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::MemFifo { pid, pages } => {
            if let Some(mut kernel) = load_kernel()? {
                println!("\n🔍 Simulando accesos con FIFO para proceso {}", pid);
                for page in pages {
                    let _ = kernel.access_memory_fifo(pid, page);
                }
                kernel.display_memory();
                kernel.status(); // Mostrar estado general, que incluye métricas de memoria
                save_kernel(&kernel)?;
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::MemLru { pid, pages } => {
            if let Some(mut kernel) = load_kernel()? {
                println!("\n🔍 Simulando accesos con LRU para proceso {}", pid);
                for page in pages {
                    let _ = kernel.access_memory_lru(pid, page);
                }
                kernel.display_memory();
                kernel.status(); // Mostrar estado general, que incluye métricas de memoria
                save_kernel(&kernel)?;
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::MemWs { pid, window, pages } => {
            if let Some(mut kernel) = load_kernel()? {
                println!("\n🔍 Simulando accesos con Working Set para proceso {} (ventana={})", pid, window);
                for page in pages {
                    let _ = kernel.access_memory_ws(pid, page, window);
                }
                kernel.display_memory();
                kernel.status();
                save_kernel(&kernel)?;
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::MemDisplay => {
            if let Some(kernel) = load_kernel()? {
                kernel.display_memory();
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::HeapAlloc { pid, size } => {
            if let Some(mut kernel) = load_kernel()? {
                match kernel.heap_alloc(pid, size) {
                    Ok(address) => {
                        println!("✅ Heap: Asignado {} bytes para proceso {} en dirección {:#x}", size, pid, address);
                        save_kernel(&kernel)?;
                    }
                    Err(e) => eprintln!("❌ Error: {}", e),
                }
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::HeapFree { address } => {
            if let Some(mut kernel) = load_kernel()? {
                match kernel.heap_free(address) {
                    Ok(_) => {
                        println!("✅ Heap: Liberada memoria en dirección {:#x}", address);
                        save_kernel(&kernel)?;
                    }
                    Err(e) => eprintln!("❌ Error: {}", e),
                }
            } else {
                eprintln!("❌ Kernel no inicializado");
            }
        }

        Commands::HeapStatus => {
            if let Some(kernel) = load_kernel()? {
                kernel.heap_status();
            } else {
                eprintln!("❌ Kernel no inicializado");
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
            let mut scan = ScanScheduler::new(ScanDirection::Up);
            
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
            println!("═══════════════════════════════════════");

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
            let mut scan = ScanScheduler::new(ScanDirection::Up);
            for (idx, cyl) in cylinders.iter().enumerate() {
                scan.add_request(DiskRequest {
                    pid: idx as u32 + 1,
                    cylinder: *cyl,
                    timestamp: idx as u64,
                });
            }
            let mut sim_scan = DiskSimulator::new(start);
            sim_scan.process_all(&mut scan);
            sim_scan.visualize(max);
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
