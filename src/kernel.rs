use anyhow::Result;
use log::info;

pub fn run(config: Option<String>) -> Result<()> {
    info!("kernel-sim: arrancando (config: {:?})", config);
    // Punto de entrada de la simulación: parse config, inicializar scheduler, memory, io, etc.
    println!("(Plantilla) Simulación iniciada — implementar módulos concretos.");
    Ok(())
}