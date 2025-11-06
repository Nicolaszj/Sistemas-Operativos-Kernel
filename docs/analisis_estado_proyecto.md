# Análisis del Estado del Proyecto - Kernel Simulation

**Fecha:** Noviembre 6, 2025  
**Proyecto:** Simulación de Núcleo de Sistema Operativo  
**Lenguaje:** Rust  
**Estudiante:** Nicolaszj

---

## 1. ARQUITECTURA SELECCIONADA

### Stack Tecnológico

- **Lenguaje:** Rust (edición 2021) ✅
- **Interfaz:** CLI (Command Line Interface) con `clap` ✅
- **Dependencias principales:**
  - `clap v4.3` - CLI parsing
  - `serde v1.0` + `serde_json v1.0` - Serialización/deserialización
  - `log v0.4` + `env_logger v0.10` - Logging
  - `anyhow v1.0` - Manejo de errores

### Diseño Arquitectónico

**Patrón:** Trait-based abstraction (Programación basada en traits)

```
kernel-sim/
├── cli/                    → Comandos CLI (en main.rs)
├── kernel                  → Orquestador principal
├── scheduler               → Planificación de procesos (Trait + impls)
├── process                 → Modelo PCB y estados
└── modules/
    ├── cpu/               → Ejecución de instrucciones
    ├── mem/               → Gestión de memoria
    ├── io/                → Entrada/salida
    ├── disk/              → Planificación de disco
    └── ipc/               → Sincronización (semáforos/mutex)
```

**Contratos entre módulos:**

- `Scheduler` trait con múltiples implementaciones intercambiables
- Logging con timestamps simulados
- Exportación de trazas en JSON para análisis

---

## 2. ESTADO ACTUAL POR COMPONENTE

### ✅ COMPONENTES COMPLETADOS

#### 1. Documentación Base (Parte 1 - COMPLETA)

- ✅ `README.md` - Objetivo general y descripción
- ✅ `docs/alcance.md` - Alcance, supuestos y restricciones
- ✅ `docs/arquitectura.md` - Diagrama de bloques y APIs
- ✅ `docs/plan_pruebas.md` - Plan de pruebas (1-2 páginas)
- ✅ Estructura de directorios creada
- ✅ `Cargo.toml` configurado con dependencias

#### 2. CLI (Comandos básicos)

- ✅ Comando `run --config <file>` - Ejecutar simulación
- ✅ Comando `create-process --burst <n> --mem <n>` - Crear proceso
- ✅ Parser con `clap` funcional
- ✅ Logging inicializado con `env_logger`

#### 3. Modelo de Procesos (PCB)

- ✅ Estructura `Process` con:
  - `pid: u32`
  - `cpu_burst: u64`
  - `memory_size: usize`
  - `state: ProcessState`
- ✅ Estados definidos: `Ready`, `Running`, `Blocked`, `Terminated`

#### 4. Planificador FIFO

- ✅ Trait `Scheduler` definido
- ✅ `FifoScheduler` implementado con `VecDeque`
- ✅ Métodos: `push()`, `next()`
- ✅ Test unitario `fifo_order` **PASANDO**

---

### ⚠️ COMPONENTES PARCIALES (Solo esqueletos)

#### 1. Kernel (`kernel.rs`)

```rust
// ACTUAL: Solo función vacía
pub fn run(config: Option<String>) -> Result<()> {
    info!("kernel-sim: arrancando (config: {:?})", config);
    println!("(Plantilla) Simulación iniciada — implementar módulos concretos.");
    Ok(())
}
```

**Falta:** Orquestación real entre scheduler, CPU, memoria, IO

#### 2. Módulo CPU (`modules/cpu/mod.rs`)

```rust
// ACTUAL: Solo placeholder
pub fn execute() {
    println!("(cpu) ejecutar instrucción");
}
```

**Falta:** Lógica de ejecución de ráfagas

#### 3. Módulo Memoria (`modules/mem/mod.rs`)

```rust
// ACTUAL: Estructura vacía
pub struct MemoryManager;
impl MemoryManager {
    pub fn alloc(&self, _pid: u32, _size: usize) -> Result<usize> { ... }
    pub fn free(&self, _pid: u32) -> Result<()> { ... }
}
```

**Falta:** Implementación de paginación y gestión de marcos

#### 4. Módulo IO (`modules/io/mod.rs`)

```rust
// ACTUAL: Solo placeholder
pub fn enqueue_io(_pid: u32, _op: &str) { ... }
```

**Falta:** Colas de dispositivos y gestión de recursos

#### 5. Módulo Disco (`modules/disk/mod.rs`)

```rust
// ACTUAL: Retorna datos dummy
pub fn read_block(_block: u64) -> Vec<u8> {
    vec![0u8; 512]
}
```

**Falta:** Algoritmos FCFS, SSTF, SCAN

#### 6. Módulo IPC (`modules/ipc/mod.rs`)

```rust
// ACTUAL: Estructuras sin lógica
pub struct Semaphore { count: i32 }
pub struct IpcRegistry { ... }
```

**Falta:** Implementación de wait/signal y problemas canónicos

---

## 3. REQUISITOS POR ENTREGA

### 📋 PARTE 1 - Fundamentos (Primera Clase)

**Estado: 80% COMPLETO**

| Requisito                  | Estado | Notas                                |
| -------------------------- | ------ | ------------------------------------ |
| Portada                    | ✅     | README.md                            |
| Objetivo general           | ✅     | README.md                            |
| Componentes y algoritmos   | ⚠️     | Falta especificar algoritmos exactos |
| Stack (C++/Rust + CLI)     | ✅     | Rust + clap                          |
| Diagrama arquitectura      | ✅     | docs/arquitectura.md                 |
| Backlog con historias      | ❌     | **FALTA CREAR**                      |
| Repositorio con estructura | ✅     | Estructura completa                  |
| Plantillas de pruebas      | ⚠️     | Solo 1 test (FIFO)                   |
| Informe técnico borrador   | ✅     | 3 archivos en /docs                  |

**FALTA PARA PARTE 1:**

1. ❌ **Backlog completo** con historias de usuario y ≥2 criterios por componente
2. ⚠️ **Especificar algoritmos concretos** para cada módulo:
   - CPU: Round Robin + SJF
   - Memoria: FIFO/LRU + (PFF o Working Set)
   - Disco: FCFS + (SSTF o SCAN)
   - Sincronización: Semáforos + problema canónico

---

### 📋 PARTE 2 - Implementación Core (Segunda Clase)

**Estado: 15% COMPLETO**

#### Objetivos principales:

1. ✅ **Modelo de PCB** - Ya implementado
2. ❌ **Round Robin** con quantum fijo
3. ❌ **CLI extendida** - Comandos: `new`, `ps`, `tick`, `kill`, `run n`
4. ❌ **Gestor de marcos** + tabla de páginas
5. ❌ **Algoritmo FIFO** de paginación con métricas
6. ❌ **Visualización** de marcos/fallos por consola
7. ❌ **Sincronización** - Framework de semáforos/mutex
8. ❌ **Productor-consumidor** con comandos `produce`, `consume`, `stat`

**ENTREGABLES ESPERADOS AL FINAL DE CLASE 2:**

- ❌ Planificador RR funcional + CLI básica
- ❌ Métricas: tiempo de espera/retorno promedio
- ❌ Algoritmo paginación 1 (FIFO o LRU) con visualización
- ❌ Módulo sincronización + demo reproducible

**LO QUE TIENES:**

- ✅ Solo FIFO scheduler (no Round Robin)
- ✅ CLI con 2 comandos básicos (falta `ps`, `tick`, `kill`, etc.)
- ❌ Sin gestión de memoria implementada
- ❌ Sin sincronización implementada

---

### 📋 PARTE 3 - Integración y Avanzado (Tercera Clase + Final)

**Estado: 5% COMPLETO**

#### Requisitos adicionales:

**Planificación de CPU:**

- ❌ **SJF no expropiativo** como segundo algoritmo
- ❌ Documentación del diseño en `/docs`

**Memoria Virtual:**

- ❌ Implementar **segundo algoritmo** (LRU o PFF/Working Set)
- ❌ **Gráficas** (CSV → notebook): fallos vs. tamaño de marcos
- ❌ **Asignador en heap** (Buddy/Segregated) con mediciones de fragmentación

**Planificación de Disco:**

- ❌ **FCFS** (First Come First Served)
- ❌ **SSTF o SCAN**
- ❌ **Gráfico** de movimiento total por algoritmo

**Sincronización:**

- ❌ **Cena de los filósofos** como caso de estudio
- ❌ Documentar invariantes

**Interfaz:**

- ❌ **Vista de marcos** de memoria (color por hits/fallos)
- ❌ **Vista de disco** (línea de cilindros + cabezal)
- ❌ **Panel de procesos/planificador** (RR/SJF)

**Entregables finales:**

1. ❌ **Scripts de reproducción:**

   - `scripts/mem_*.txt` (trazas de memoria)
   - `scripts/disk_*.txt` (secuencias de cilindros)
   - `scripts/proc_*.txt` (llegadas y ráfagas)

2. ❌ **Informe técnico completo** con:

   - Memoria virtual: 2+ algoritmos (incluye PFF o Working Set)
   - Asignador heap: diseño, fragmentación, latencia
   - Disco: comparativa FCFS vs. SSTF/SCAN
   - Sincronización: diseño e invariantes
   - Diseño de interfaz: capturas y flujo
   - Conclusiones: trade-offs de cada algoritmo

3. ❌ **Diagrama de módulos y flujos** de procesos

4. ⚠️ **Sustentación** (50% de la nota):
   - Video corto o guion de 5 minutos

---

## 4. ALGORITMOS REQUERIDOS (Resumen)

### Gestión de Procesos (≥2 algoritmos)

- ❌ **Round Robin** (RR) con quantum fijo
- ❌ **Shortest Job First** (SJF) no expropiativo

### Memoria Virtual (≥2 algoritmos + 1 avanzado)

- ❌ **FIFO** (First In First Out)
- ❌ **LRU** (Least Recently Used)
- ❌ **PFF** (Page Fault Frequency) **O** **Working Set** (elegir uno)

### Planificación de Disco (≥2 algoritmos)

- ❌ **FCFS** (First Come First Served)
- ❌ **SSTF** (Shortest Seek Time First) **O** **SCAN** (elegir uno)

### Sincronización (≥2 problemas)

- ❌ **Productor-Consumidor**
- ❌ **Cena de los Filósofos**
- Opcional: Lectores-Escritores

---

## 5. PRIORIDADES DE IMPLEMENTACIÓN

### 🔴 URGENTE (Para Parte 2)

1. **Round Robin Scheduler**

   - Implementar `RoundRobinScheduler` con quantum configurable
   - Agregar campo `remaining_quantum` a `Process`
   - Test: validar fairness y tiempos de espera/retorno

2. **CLI Extendida**

   - `new <burst> <mem>` - Crear proceso y agregarlo al scheduler
   - `ps` - Listar procesos con estado
   - `tick [n]` - Avanzar n pasos de simulación
   - `kill <pid>` - Terminar proceso
   - `run <n>` - Ejecutar n pasos completos

3. **Gestor de Memoria - Paginación FIFO**

   - Estructura `PageTable` por proceso
   - Estructura `FrameManager` con cola FIFO
   - Métricas: fallos totales, tasa de aciertos
   - Visualización ASCII de marcos

4. **Sincronización Básica**
   - Implementar `Semaphore::wait()` y `Semaphore::signal()`
   - Problema productor-consumidor con buffer simulado
   - Comandos CLI: `produce <item>`, `consume`, `stat`

### 🟡 IMPORTANTE (Para Parte 3)

5. **SJF Scheduler**

   - Ordenar por `cpu_burst` más corto
   - Documentar supuestos (burst conocido vs. estimado)

6. **Algoritmo de Paginación LRU/PFF**

   - Implementar segundo algoritmo
   - Comparativa con gráficos (CSV export)

7. **Planificación de Disco**

   - `DiskScheduler` trait
   - Implementaciones: FCFS + SSTF/SCAN
   - Visualización de movimiento del cabezal

8. **Cena de los Filósofos**
   - 5 filósofos, 5 tenedores (semáforos)
   - Prevenir deadlock/starvation

### 🟢 OPCIONAL (Valor agregado)

9. **Asignador en Heap**

   - Buddy System o Segregated Free Lists
   - Métricas de fragmentación interna/externa

10. **Visualización Avanzada**

    - Colores en terminal (crate `colored`)
    - Gráficos con Python/Jupyter desde CSVs

11. **Scripts de Reproducción**
    - Archivos en `scripts/` para experimentos repetibles

---

## 6. GAPS CRÍTICOS IDENTIFICADOS

### 📊 Cobertura de Requisitos

| Componente          | Requisito         | Implementado                      | Porcentaje |
| ------------------- | ----------------- | --------------------------------- | ---------- |
| **CPU Scheduling**  | 2 algoritmos      | 0.5/2 (solo FIFO, falta RR y SJF) | **25%**    |
| **Memoria Virtual** | 3 algoritmos      | 0/3                               | **0%**     |
| **Disco**           | 2 algoritmos      | 0/2                               | **0%**     |
| **Sincronización**  | 2 problemas       | 0/2                               | **0%**     |
| **CLI**             | 6+ comandos       | 2/6                               | **33%**    |
| **Testing**         | 4+ tipos          | 1/4                               | **25%**    |
| **Documentación**   | Backlog + informe | 3/5 docs                          | **60%**    |
| **Scripts**         | 3 tipos           | 0/3                               | **0%**     |

**COBERTURA GLOBAL: ~18%**

### ⚠️ Problemas Detectados

1. **FIFO vs. Round Robin**: Tienes FIFO implementado, pero el requisito Parte 2 pide **Round Robin**. FIFO no está en la lista de algoritmos requeridos para CPU.

2. **Sin orquestación**: El `kernel.rs` no coordina ningún módulo realmente.

3. **Sin métricas**: No hay logging de tiempos de espera, retorno, fallos de página, etc.

4. **Sin visualización**: Los requisitos piden mostrar marcos, disco, procesos en consola.

5. **Sin backlog**: Falta documento con historias de usuario y criterios de aceptación.

6. **Sin scripts**: No hay archivos de entrada para reproducir experimentos.

---

## 7. RECOMENDACIONES

### Para aprobar Parte 2 (próxima clase):

1. **Convertir FIFO a Round Robin** o crear `RoundRobinScheduler` nuevo
2. **Implementar CLI completa** con `ps`, `tick`, `kill`, `run`
3. **Crear gestor de paginación FIFO** con métricas y visualización
4. **Implementar semáforos** + productor-consumidor básico
5. **Agregar tests** para RR y paginación

### Para completar Parte 3 (entrega final):

6. Implementar **SJF** scheduler
7. Agregar **LRU o PFF** para memoria
8. Implementar **FCFS + SSTF/SCAN** para disco
9. Resolver **Cena de los Filósofos**
10. Crear **scripts de reproducción** en `/scripts`
11. Completar **informe técnico** con gráficos comparativos
12. Preparar **sustentación** (video/guion de 5 min)

### Criterios de Evaluación (50% entregables):

- Implementación módulos clave: **40%**
- Integración entre componentes: **10%**
- Calidad informe técnico: **20%**
- Pruebas y resultados: **10%**
- Documentación código: **10%**
- Valor agregado: **10%**

---

## 8. CONCLUSIÓN

**Estado actual:** Tienes una **base sólida** (arquitectura, estructura, docs) pero **muy poca implementación funcional**.

**Lo más urgente:**

1. ✅ FIFO funciona → convertirlo a **Round Robin**
2. ❌ Memoria → implementar **paginación FIFO con métricas**
3. ❌ CLI → agregar comandos `ps`, `tick`, `kill`
4. ❌ Sincronización → semáforos + productor-consumidor

**Siguiente entrega (Parte 2):** Necesitas tener al menos **4 componentes funcionando**:

- RR scheduler con métricas
- Paginación FIFO con visualización
- CLI extendida (6 comandos)
- Sincronización básica (semáforos)

**Tiempo estimado:** Si trabajas enfocado, puedes completar Parte 2 en **8-12 horas** de desarrollo.

---

**Generado:** 2025-11-06  
**Repositorio:** Sistemas-Operativos-Kernel  
**Rama:** main
