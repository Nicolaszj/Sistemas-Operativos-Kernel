# Scripts de Prueba - Kernel Simulation

Este directorio contiene scripts para reproducir experimentos del proyecto.

## 📁 Estructura

### Pruebas de Memoria

- `mem_test1_fifo.txt` - Secuencia que demuestra anomalía de Belady con FIFO
- `mem_test2_lru.txt` - Secuencia con localidad temporal para LRU

### Pruebas de Disco

- `disk_fcfs.txt` - Ejemplo clásico de FCFS (643 cilindros de movimiento)
- `disk_scan.txt` - Comparativa FCFS vs SSTF vs SCAN

### Escenarios de Procesos

- `proc_scenario1.txt` - Procesos CPU-bound (comparar RR vs SJF)
- `proc_scenario2.txt` - Procesos con I/O (demostrar bloqueo/multiprogramación)

---

## 🚀 Cómo Usar

### Memoria - FIFO

```bash
# Inicializar kernel con 4 marcos
cargo run -- init --frames 4

# Crear proceso 1
cargo run -- new --burst 10 --mem 100

# Simular accesos a memoria con FIFO
cargo run -- mem-fifo 1 1 2 3 4 1 2 5 1 2 3 4 5

# Ver marcos de memoria
cargo run -- mem-display
```

**Resultado esperado:**

- Fallos: 9
- Hits: 3
- Tasa de aciertos: 25%

---

### Memoria - LRU

```bash
# Inicializar kernel
cargo run -- init --frames 4

# Crear proceso
cargo run -- new --burst 10 --mem 100

# Simular accesos con LRU
cargo run -- mem-lru 1 1 2 3 4 1 2 1 2 1 2 3 5

# Ver estado
cargo run -- mem-display
```

**Resultado esperado:**

- Fallos: 5
- Hits: 7
- Tasa de aciertos: 58.3%
- ✅ LRU tiene mejor rendimiento que FIFO en esta secuencia

---

### Disco - FCFS

```bash
# Simular con FCFS
cargo run -- disk-fcfs --start 50 98 183 37 122 14 124 65 67
```

**Resultado esperado:**

- Movimiento total: 643 cilindros
- Promedio: 80.4 cilindros/solicitud

---

### Disco - SSTF

```bash
# Simular con SSTF
cargo run -- disk-sstf --start 50 98 183 37 122 14 124 65 67
```

**Resultado esperado:**

- Movimiento total: 239 cilindros
- Promedio: 29.9 cilindros/solicitud
- ✅ 62.8% mejor que FCFS

---

### Disco - SCAN

```bash
# Simular con SCAN
cargo run -- disk-scan --start 50 --max 199 98 183 37 122 14 124 65 67
```

**Resultado esperado:**

- Movimiento total: 302 cilindros
- Promedio: 37.8 cilindros/solicitud
- ✅ 53% mejor que FCFS

---

### Disco - Comparativa

```bash
# Comparar los 3 algoritmos simultáneamente
cargo run -- disk-compare --start 50 --max 199 98 183 37 122 14 124 65 67
```

**Resultado esperado:**

```
╔════════════════════════════════════════════════╗
║         RESUMEN COMPARATIVO                    ║
╠════════════╦═══════════════╦══════════════════╣
║ Algoritmo  ║   Movimiento  ║   Eficiencia     ║
╠════════════╬═══════════════╬══════════════════╣
║ FCFS       ║     643       ║    Baseline      ║
║ SSTF       ║     239       ║    +62.8%        ║
║ SCAN       ║     302       ║    +53.0%        ║
╚════════════╩═══════════════╩══════════════════╝

🏆 Mejor algoritmo: SSTF (movimiento: 239)
```

---

### Procesos - Escenario 1 (CPU-bound)

```bash
# Inicializar con Round Robin (quantum=3)
cargo run -- init --scheduler rr --quantum 3

# Crear procesos según escenario 1
cargo run -- new --burst 10 --mem 100  # P1
cargo run -- new --burst 3 --mem 50    # P2
cargo run -- new --burst 6 --mem 80    # P3
cargo run -- new --burst 1 --mem 40    # P4
cargo run -- new --burst 12 --mem 120  # P5

# Ejecutar simulación
cargo run -- run 35

# Ver métricas
cargo run -- metrics
```

**Comparar con SJF:**

```bash
# Inicializar con SJF
cargo run -- init --scheduler sjf

# Crear los mismos procesos
cargo run -- new --burst 10 --mem 100
cargo run -- new --burst 3 --mem 50
cargo run -- new --burst 6 --mem 80
cargo run -- new --burst 1 --mem 40
cargo run -- new --burst 12 --mem 120

# Ejecutar
cargo run -- run 35

# Comparar métricas
cargo run -- metrics
```

**Resultados esperados:**

- RR: Espera promedio = 9.4, Turnaround = 15.8
- SJF: Espera promedio = 7.6 ✅ MEJOR, Turnaround = 14.0 ✅ MEJOR

---

### Sincronización - Productor-Consumidor

```bash
# Inicializar
cargo run -- init

# Producir items
cargo run -- produce "Item1" --pid 100
cargo run -- produce "Item2" --pid 100
cargo run -- produce "Item3" --pid 100

# Ver estado del buffer
cargo run -- buffer-stat

# Consumir
cargo run -- consume --pid 200
cargo run -- consume --pid 200

# Ver estado actualizado
cargo run -- buffer-stat
```

---

### Sincronización - Filósofos

```bash
# Simular cena de 5 filósofos por 10 pasos
cargo run -- philosophers --count 5 --steps 10
```

**Resultado esperado:**

- Los 5 filósofos comen sin deadlock
- No hay inanición (todos comen al menos una vez)
- Solución con orden asimétrico funciona correctamente

---

## 📊 Exportar Datos

Para generar gráficos, puedes redirigir la salida a archivos CSV:

```bash
# Memoria
cargo run -- mem-fifo 1 1 2 3 4 1 2 5 > results/mem_fifo.txt
cargo run -- mem-lru 1 1 2 3 4 1 2 5 > results/mem_lru.txt

# Disco
cargo run -- disk-compare --start 50 98 183 37 122 > results/disk_compare.txt
```

Luego usa el script Python (próximamente en `/scripts`) para generar gráficos.

---

## 🔬 Experimentos Sugeridos

### 1. Anomalía de Belady

Comparar FIFO con 3 marcos vs 4 marcos:

- Secuencia: 1,2,3,4,1,2,5,1,2,3,4,5
- Hipótesis: 4 marcos → MÁS fallos que 3 marcos

### 2. Localidad Temporal

Comparar FIFO vs LRU con secuencia repetitiva:

- Secuencia: 1,2,3,1,2,3,1,2,3,4
- Hipótesis: LRU >> FIFO

### 3. Planificación Justa

Ejecutar 3 procesos (burst: 10, 1, 10) con RR vs SJF:

- RR: Todos progresan equitativamente
- SJF: Proceso corto termina primero, largos esperan

### 4. Deadlock en Filósofos

Modificar código para permitir tomar tenedores en cualquier orden:

- Hipótesis: Ocurre deadlock
- Solución actual: Orden asimétrico previene deadlock

---

## 📝 Formato de Scripts

### Memoria

```
PID: <process_id>
PAGES: <page1>,<page2>,...
FRAMES: <num_frames>
```

### Disco

```
CYLINDERS: <cyl1>,<cyl2>,...
START_POSITION: <initial_position>
```

### Procesos

```
# tiempo_llegada, cpu_burst, memoria
<t1>, <burst1>, <mem1>
<t2>, <burst2>, <mem2>
...
```

---

**Última actualización:** 2025-11-06  
**Proyecto:** Sistemas Operativos - Kernel Simulation
