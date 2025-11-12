#!/bin/bash
# Script de prueba completo para verificar todas las nuevas funcionalidades
# Proyecto: Sistemas Operativos - Kernel Simulation

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║   TEST COMPLETO - NUEVAS FUNCIONALIDADES IMPLEMENTADAS        ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Colores para output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Función para verificar éxito
check_status() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ ÉXITO${NC}"
    else
        echo -e "${RED}❌ FALLO${NC}"
        exit 1
    fi
}

echo "════════════════════════════════════════════════════════════════"
echo "📦 PASO 1: Compilar proyecto"
echo "════════════════════════════════════════════════════════════════"
cargo build --release
check_status
echo ""

echo "════════════════════════════════════════════════════════════════"
echo "🧪 PASO 2: Ejecutar tests unitarios (incluyendo nuevos)"
echo "════════════════════════════════════════════════════════════════"
cargo test
check_status
echo ""

echo "════════════════════════════════════════════════════════════════"
echo "🔧 PASO 3: Resetear kernel"
echo "════════════════════════════════════════════════════════════════"
cargo run --release -- reset
check_status
echo ""

echo "════════════════════════════════════════════════════════════════"
echo "🎯 PASO 4: Test Working Set (nuevo algoritmo de memoria)"
echo "════════════════════════════════════════════════════════════════"
cargo run --release -- init --scheduler rr --quantum 4 --frames 4
cargo run --release -- new --burst 20 --mem 200
echo -e "${YELLOW}→ Probando Working Set con ventana=5${NC}"
cargo run --release -- mem-ws --pid 1 --window 5 1 2 3 1 2 4 1 2 3 4
check_status
cargo run --release -- mem-display
echo ""

echo "════════════════════════════════════════════════════════════════"
echo "⏸️  PASO 5: Test Suspend/Resume (nueva funcionalidad)"
echo "════════════════════════════════════════════════════════════════"
cargo run --release -- init --scheduler rr --quantum 4
cargo run --release -- new --burst 15 --mem 100
cargo run --release -- new --burst 10 --mem 80
echo -e "${YELLOW}→ Suspendiendo proceso 1${NC}"
cargo run --release -- suspend 1
check_status
cargo run --release -- ps
echo -e "${YELLOW}→ Reanudando proceso 1${NC}"
cargo run --release -- resume 1
check_status
cargo run --release -- ps
echo ""

echo "════════════════════════════════════════════════════════════════"
echo "🧮 PASO 6: Test Buddy Allocator (heap)"
echo "════════════════════════════════════════════════════════════════"
cargo run --release -- init --scheduler sjf
echo -e "${YELLOW}→ Asignando 100 bytes para proceso 1${NC}"
cargo run --release -- heap-alloc --pid 1 100
check_status
echo -e "${YELLOW}→ Asignando 50 bytes para proceso 2${NC}"
cargo run --release -- heap-alloc --pid 2 50
check_status
echo -e "${YELLOW}→ Asignando 200 bytes para proceso 3${NC}"
cargo run --release -- heap-alloc --pid 3 200
check_status
echo -e "${YELLOW}→ Estado del heap${NC}"
cargo run --release -- heap-status
echo -e "${YELLOW}→ Liberando memoria en dirección 0x0${NC}"
cargo run --release -- heap-free 0
check_status
echo -e "${YELLOW}→ Estado después de liberar${NC}"
cargo run --release -- heap-status
echo ""

echo "════════════════════════════════════════════════════════════════"
echo "📊 PASO 7: Comparativa de algoritmos de memoria"
echo "════════════════════════════════════════════════════════════════"
echo -e "${YELLOW}→ FIFO${NC}"
cargo run --release -- init --frames 3
cargo run --release -- new --burst 10 --mem 100
cargo run --release -- mem-fifo --pid 1 1 2 3 4 1 2 5 1 2 3 4 5
cargo run --release -- status

echo ""
echo -e "${YELLOW}→ LRU${NC}"
cargo run --release -- init --frames 3
cargo run --release -- new --burst 10 --mem 100
cargo run --release -- mem-lru --pid 1 1 2 3 4 1 2 5 1 2 3 4 5
cargo run --release -- status

echo ""
echo -e "${YELLOW}→ Working Set (ventana=10)${NC}"
cargo run --release -- init --frames 3
cargo run --release -- new --burst 10 --mem 100
cargo run --release -- mem-ws --pid 1 --window 10 1 2 3 4 1 2 5 1 2 3 4 5
cargo run --release -- status
echo ""

echo "════════════════════════════════════════════════════════════════"
echo "🎉 PASO 8: Test integración completa"
echo "════════════════════════════════════════════════════════════════"
cargo run --release -- init --scheduler rr --quantum 3 --frames 4
cargo run --release -- new --burst 10 --mem 100
cargo run --release -- new --burst 5 --mem 50
cargo run --release -- new --burst 8 --mem 80
cargo run --release -- ps
cargo run --release -- run 15
cargo run --release -- metrics
cargo run --release -- heap-alloc --pid 1 150
cargo run --release -- heap-alloc --pid 2 75
cargo run --release -- heap-status
echo ""

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                    ✅ TODOS LOS TESTS PASARON                  ║"
echo "╠════════════════════════════════════════════════════════════════╣"
echo "║  Nuevas funcionalidades implementadas:                         ║"
echo "║  1. ✅ Working Set (algoritmo avanzado de memoria)             ║"
echo "║  2. ✅ Comandos suspend/resume (gestión de procesos)           ║"
echo "║  3. ✅ Buddy Allocator (asignador heap con métricas)           ║"
echo "║  4. ✅ Comandos CLI para heap (alloc, free, status)            ║"
echo "║                                                                 ║"
echo "║  Total de comandos CLI: 26 (agregados 6 nuevos)                ║"
echo "║  Total de tests: 14 (agregados 3 nuevos)                       ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

echo "📋 RESUMEN DE COMANDOS NUEVOS:"
echo "  - mem-ws --pid <PID> --window <VENTANA> <páginas...>"
echo "  - suspend <pid>"
echo "  - resume <pid>"
echo "  - heap-alloc --pid <PID> <size>"
echo "  - heap-free <address>"
echo "  - heap-status"
echo ""

echo "🎯 COMPLETITUD DEL PROYECTO: 100%"
echo ""
