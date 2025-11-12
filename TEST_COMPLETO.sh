# PRUEBA COMPLETA DEL KERNEL SIMULATOR
# Copia y pega este bloque completo en PowerShell (dentro de WSL)

# Navegar al directorio y compilar
cd /mnt/c/Users/local1/Sistemas-Operativos-Kernel && cargo build --release 2>&1 | tail -n 3

# Ejecutar todas las pruebas
echo "=========================================="
echo "INICIANDO PRUEBA COMPLETA"
echo "=========================================="

# 1. Reset
echo -e "\n[1/10] RESET..."
./target/release/kernel-sim reset

# 2. Init
echo -e "\n[2/10] INIT (Round Robin, quantum=3, 8 frames)..."
./target/release/kernel-sim init --scheduler rr --quantum 3 --frames 8

# 3. Crear procesos
echo -e "\n[3/10] CREAR PROCESOS..."
./target/release/kernel-sim new --burst 10 --mem 5
./target/release/kernel-sim new --burst 15 --mem 8
./target/release/kernel-sim new --burst 8 --mem 6
./target/release/kernel-sim new --burst 20 --mem 10

# 4. Listar procesos
echo -e "\n[4/10] LISTAR PROCESOS..."
./target/release/kernel-sim ps

# 5. Ejecutar scheduler
echo -e "\n[5/10] EJECUTAR SCHEDULER (5 ciclos)..."
./target/release/kernel-sim run 5

# 6. Suspend/Resume
echo -e "\n[6/10] SUSPEND/RESUME proceso 2..."
./target/release/kernel-sim suspend 2
echo "Estado después de suspend:"
./target/release/kernel-sim ps
./target/release/kernel-sim resume 2
echo "Estado después de resume:"
./target/release/kernel-sim ps

# 7. Memoria FIFO
echo -e "\n[7/10] MEMORIA VIRTUAL (FIFO)..."
./target/release/kernel-sim mem-fifo --pid 1 0 1 2 3
./target/release/kernel-sim mem-display

# 8. Memoria LRU
echo -e "\n[8/10] MEMORIA VIRTUAL (LRU)..."
./target/release/kernel-sim mem-lru --pid 2 0 1 0
./target/release/kernel-sim mem-display

# 9. Buddy Allocator
echo -e "\n[9/10] BUDDY ALLOCATOR..."
./target/release/kernel-sim heap-alloc --pid 1 64
./target/release/kernel-sim heap-alloc --pid 2 128
./target/release/kernel-sim heap-alloc --pid 3 32
./target/release/kernel-sim heap-status
./target/release/kernel-sim heap-free 0
echo "Después de free:"
./target/release/kernel-sim heap-status

# 10. Disk Scheduler
echo -e "\n[10/10] DISK SCHEDULER (SCAN)..."
./target/release/kernel-sim disk-scan --start 50 20 80 10 90 30

# EXTRA: Producer/Consumer
echo -e "\n[EXTRA-1] PRODUCER/CONSUMER..."
./target/release/kernel-sim produce "Dato1"
./target/release/kernel-sim produce "Dato2"
./target/release/kernel-sim consume
./target/release/kernel-sim buffer-stat

# EXTRA: Dining Philosophers
echo -e "\n[EXTRA-2] DINING PHILOSOPHERS (5 filósofos, 3 iteraciones)..."
./target/release/kernel-sim philosophers --count 5 --steps 3

echo -e "\n=========================================="
echo "PRUEBA COMPLETA FINALIZADA"
echo "=========================================="
