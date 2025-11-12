#!/bin/bash
# Script de prueba completa del kernel simulator
# Copia y pega todo este bloque en la terminal WSL

echo "=========================================="
echo "PRUEBA COMPLETA DEL KERNEL SIMULATOR"
echo "=========================================="
echo ""

# Limpiar estado anterior
echo "[1/10] Limpiando estado anterior..."
./target/debug/kernel-sim reset
echo ""

# Inicializar kernel con Round Robin
echo "[2/10] Inicializando kernel (RR, quantum=3, 8 frames)..."
./target/debug/kernel-sim init --scheduler rr --quantum 3 --frames 8
echo ""

# Crear procesos
echo "[3/10] Creando procesos..."
./target/debug/kernel-sim new --burst 10 --mem 5
./target/debug/kernel-sim new --burst 15 --mem 8
./target/debug/kernel-sim new --burst 8 --mem 6
./target/debug/kernel-sim new --burst 20 --mem 10
echo ""

# Ver estado de procesos
echo "[4/10] Listando procesos..."
./target/debug/kernel-sim ps
echo ""

# Ejecutar scheduler
echo "[5/10] Ejecutando scheduler (5 ciclos)..."
./target/debug/kernel-sim run --cycles 5
echo ""

# Probar suspend/resume
echo "[6/10] Probando suspend/resume..."
./target/debug/kernel-sim suspend --pid 2
./target/debug/kernel-sim ps
./target/debug/kernel-sim resume --pid 2
./target/debug/kernel-sim ps
echo ""

# Probar memoria con FIFO
echo "[7/10] Probando memoria virtual (FIFO)..."
./target/debug/kernel-sim mem-fifo --pid 1 --page 0
./target/debug/kernel-sim mem-fifo --pid 1 --page 1
./target/debug/kernel-sim mem-fifo --pid 1 --page 2
./target/debug/kernel-sim mem-fifo --pid 1 --page 3
./target/debug/kernel-sim mem-show
echo ""

# Probar memoria con LRU
echo "[8/10] Probando memoria virtual (LRU)..."
./target/debug/kernel-sim mem-lru --pid 2 --page 0
./target/debug/kernel-sim mem-lru --pid 2 --page 1
./target/debug/kernel-sim mem-lru --pid 2 --page 0
./target/debug/kernel-sim mem-show
echo ""

# Probar Buddy Allocator
echo "[9/10] Probando Buddy Allocator..."
./target/debug/kernel-sim heap-alloc --size 64
./target/debug/kernel-sim heap-alloc --size 128
./target/debug/kernel-sim heap-alloc --size 32
./target/debug/kernel-sim heap-status
./target/debug/kernel-sim heap-free --addr 0
./target/debug/kernel-sim heap-status
echo ""

# Probar disk scheduler con SCAN
echo "[10/10] Probando disk scheduler (SCAN)..."
./target/debug/kernel-sim disk-scan --track 50
./target/debug/kernel-sim disk-scan --track 20
./target/debug/kernel-sim disk-scan --track 80
./target/debug/kernel-sim disk-scan --track 10
./target/debug/kernel-sim disk-info
echo ""

# IPC - Producer/Consumer
echo "[EXTRA] Probando Producer/Consumer..."
./target/debug/kernel-sim ipc-produce --item "Dato1"
./target/debug/kernel-sim ipc-produce --item "Dato2"
./target/debug/kernel-sim ipc-consume
./target/debug/kernel-sim ipc-status
echo ""

# Dining Philosophers
echo "[EXTRA] Probando Dining Philosophers (3 iteraciones)..."
./target/debug/kernel-sim philosophers --iterations 3
echo ""

echo "=========================================="
echo "PRUEBA COMPLETA FINALIZADA"
echo "=========================================="
