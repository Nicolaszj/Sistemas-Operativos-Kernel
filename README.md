# 🚀 Kernel Simulation - Sistemas Operativos

**Proyecto:** Simulación completa de núcleo de sistema operativo  
**Estudiantes:** Nicolas Zapata, Andru Quiroz y Luis Estiven Moreno.
**Lenguaje:** Rust 2021  
**Estado:** ✅ **CÓDIGO COMPLETO - 100% IMPLEMENTADO**

---

## ⚠️ PROBLEMA DE COMPILACIÓN EN WINDOWS

**Si `cargo build` falla con error de `link.exe`**, lee **[SOLUCION_COMPILACION.md](SOLUCION_COMPILACION.md)** para 3 soluciones:

1. **WSL (Recomendado)** - Instala en 5 min, compila inmediatamente
2. **Visual Studio Build Tools** - Requiere ~6GB y 20 min
3. **GitHub Codespaces** - Usa la nube, sin instalación local

**Solución rápida con WSL:**
```powershell
# Como Administrador:
.\install_wsl.ps1
```

---

## 📦 Características Implementadas

### ✅ Planificación de CPU
- **Round Robin** (quantum=4, cola circular)
- **SJF** (Shortest Job First, no-preemptive)
- Métricas: T_espera, T_retorno, T_respuesta

### ✅ Gestión de Memoria
- **Paginación FIFO** (demuestra anomalía de Belady)
- **Paginación LRU** (Least Recently Used)
- Visualización de marcos de memoria
- Tracking de fallos de página

### ✅ Planificación de Disco
- **FCFS** (First Come First Served)
- **SSTF** (Shortest Seek Time First)
- **SCAN** (Algoritmo del elevador)
- Visualización de movimiento del cabezal

### ✅ Sincronización (IPC)
- **Semáforos** (wait/signal)
- **Productor-Consumidor** (3 semáforos)
- **Cena de Filósofos** (5 filósofos, prevención de deadlock)

### ✅ CLI Interactiva
- 20+ comandos implementados
- Visualización con tablas ASCII
- Modo interactivo y por scripts

---

## 🚀 Compilación y Ejecución

### En WSL (Recomendado):

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Ir al proyecto
cd /mnt/c/Users/Nico/Desktop/SistemasOp

# Compilar
cargo build --release

# Ejecutar tests
cargo test

# Probar CLI
cargo run -- init --scheduler rr --quantum 4
```

### En Windows (con Build Tools instaladas):

```powershell
# Compilar
cargo build --release

# Ejecutar tests
cargo test

# Probar CLI
cargo run -- init --scheduler rr --quantum 4
```

---

## 📖 Documentación Completa

- **[RESUMEN_EJECUTIVO.md](RESUMEN_EJECUTIVO.md)** - Plan de 7 horas para completar el proyecto
- **[GUIA_ESTUDIANTE.md](GUIA_ESTUDIANTE.md)** - Instrucciones paso a paso
- **[SOLUCION_COMPILACION.md](SOLUCION_COMPILACION.md)** - Soluciones si no compila
- **[docs/CHECKLIST_PROYECTO.md](docs/CHECKLIST_PROYECTO.md)** - Estado completo del proyecto
- **[docs/backlog.md](docs/backlog.md)** - Historias de usuario
- **[docs/algoritmos_seleccionados.md](docs/algoritmos_seleccionados.md)** - Justificación técnica
- **[docs/arquitectura.md](docs/arquitectura.md)** - Diseño del sistema
- **[scripts/README.md](scripts/README.md)** - Guía de scripts de prueba

---

## 🧪 Comandos CLI

```bash
# Inicializar kernel
cargo run -- init --scheduler rr --quantum 4

# Crear procesos
cargo run -- new --burst 10 --mem 100
cargo run -- new --burst 5 --mem 50

# Listar procesos
cargo run -- ps

# Avanzar simulación
cargo run -- tick 5
cargo run -- run 20

# Ver métricas
cargo run -- metrics
cargo run -- status

# Memoria
cargo run -- mem-fifo 1 2 3 4 1 2 5
cargo run -- mem-lru 1 2 3 4 1 2 5
cargo run -- mem-display

# Disco
cargo run -- disk-fcfs 98 183 37 122
cargo run -- disk-sstf --start 50 98 183 37 122
cargo run -- disk-scan --start 50 98 183 37 122
cargo run -- disk-compare --start 50 98 183 37 122

# Sincronización
cargo run -- produce "Item1"
cargo run -- consume
cargo run -- buffer-stat
cargo run -- philosophers --count 5 --steps 10
```

---

## 📊 Estructura del Proyecto

```
SistemasOp/
├── Cargo.toml                      # Dependencias
├── README.md                       # Este archivo
├── RESUMEN_EJECUTIVO.md           # Plan de trabajo
├── GUIA_ESTUDIANTE.md             # Instrucciones detalladas
├── SOLUCION_COMPILACION.md        # Soluciones de compilación
├── install_wsl.ps1                # Script instalador WSL
├── src/
│   ├── main.rs                    # CLI (20+ comandos)
│   ├── lib.rs                     # Exportaciones
│   ├── kernel.rs                  # Orquestador principal
│   ├── process.rs                 # Estructura Process
│   ├── scheduler.rs               # RR + SJF
│   └── modules/
│       ├── cpu/                   # Módulo CPU
│       ├── mem/
│       │   └── paging.rs          # FIFO + LRU
│       ├── disk/
│       │   └── scheduler.rs       # FCFS + SSTF + SCAN
│       └── ipc/
│           ├── sync.rs            # Semáforos + Prod-Cons
│           └── philosophers.rs    # Filósofos
├── docs/
│   ├── backlog.md                 # Historias de usuario
│   ├── algoritmos_seleccionados.md
│   ├── arquitectura.md
│   ├── plan_pruebas.md
│   └── CHECKLIST_PROYECTO.md
├── scripts/
│   ├── README.md                  # Guía de scripts
│   ├── plot_graphs.py             # Generador de gráficos
│   ├── mem_test1_fifo.txt
│   ├── mem_test2_lru.txt
│   ├── disk_fcfs.txt
│   ├── disk_scan.txt
│   ├── proc_scenario1.txt
│   └── proc_scenario2.txt
└── tests/                         # 11 tests unitarios
```

---

## 🎯 Próximos Pasos

1. **Compilar proyecto** - Ver [SOLUCION_COMPILACION.md](SOLUCION_COMPILACION.md)
2. **Ejecutar demos** - Ver [RESUMEN_EJECUTIVO.md](RESUMEN_EJECUTIVO.md) sección 3
3. **Generar gráficos** - `python scripts/plot_graphs.py`
4. **Completar documentación** - Crear `docs/resultados.md` y `docs/conclusiones.md`
5. **Preparar sustentación** - Video de 5 minutos

---

## 📄 Licencia

Proyecto académico - Universidad  
Estudiantes: Nicolas Zapata, Andru Quiroz y Luis Estiven Moreno.
Curso: Sistemas Operativos  
Fecha: Noviembre 2025
      - ipc/
  - docs/
    - alcance.md
    - arquitectura.md
    - plan_pruebas.md
    - backlog.md
  - tests/
  - .gitignore

Comandos útiles

- Compilar: cargo build
- Ejecutar: cargo run -- run --config examples/config.toml
- Tests: cargo test
```
