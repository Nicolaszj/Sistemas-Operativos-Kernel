//! Disk simulation skeleton

pub fn read_block(block_id: u64) -> Vec<u8> {
    println!("(disk) lectura bloque {}", block_id);
    vec![0u8; 512] // datos simulados
}