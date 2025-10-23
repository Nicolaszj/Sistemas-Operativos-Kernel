use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "kernel-sim")]
#[command(about = "Simulación simplificada de núcleo (kernel) — CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Ejecutar una simulación (opcional: archivo de configuración)
    Run {
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Crear un proceso de ejemplo desde CLI
    CreateProcess {
        #[arg(short, long)]
        burst: u64,
        #[arg(short, long)]
        mem: usize,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { config }) => {
            println!("Iniciando simulación con config: {:?}", config);
            kernel_sim::kernel::run(config)?;
        }
        Some(Commands::CreateProcess { burst, mem }) => {
            println!("Crear proceso: burst={} mem={} (ejemplo)", burst, mem);
            // en la implementación real aquí llamarías a kernel::create_process(...)
        }
        None => {
            println!("Usa `kernel-sim --help` para ver comandos");
        }
    }

    Ok(())
}