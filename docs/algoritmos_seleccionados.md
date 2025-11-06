# Algoritmos Seleccionados - Kernel Simulation

**Proyecto:** Simulación de Núcleo de Sistema Operativo  
**Estudiante:** Nicolaszj  
**Fecha:** Noviembre 6, 2025

---

## 🎯 ALGORITMOS IMPLEMENTADOS

### 1. PLANIFICACIÓN DE PROCESOS (CPU Scheduling)

#### 1.1 Round Robin (RR)

**Tipo:** Expropiativo  
**Parámetros:** Quantum = 4 unidades de tiempo

**Funcionamiento:**

- Cada proceso recibe un quantum fijo de CPU
- Si no termina en su quantum, vuelve al final de la cola
- Garantiza fairness: todos los procesos progresan

**Ventajas:**

- ✅ Excelente tiempo de respuesta
- ✅ No hay inanición (starvation)
- ✅ Ideal para sistemas interactivos

**Desventajas:**

- ❌ Mayor overhead por cambios de contexto
- ❌ Tiempo de retorno puede ser peor que SJF

**Cuándo usar:** Sistemas time-sharing, multitarea, interfaces interactivas

---

#### 1.2 Shortest Job First (SJF)

**Tipo:** No expropiativo  
**Parámetros:** Burst conocido a priori (simplificación)

**Funcionamiento:**

- Ordena procesos por cpu_burst de menor a mayor
- Ejecuta completamente el proceso más corto
- Minimiza tiempo de espera promedio (óptimo)

**Ventajas:**

- ✅ Minimiza tiempo de espera promedio
- ✅ Óptimo en tiempo de retorno para procesos cortos

**Desventajas:**

- ❌ Puede causar inanición en procesos largos
- ❌ Requiere conocer burst (difícil en sistemas reales)

**Cuándo usar:** Batch processing, cuando se conocen duraciones, procesos no interactivos

---

### 2. GESTIÓN DE MEMORIA VIRTUAL

#### 2.1 FIFO (First In First Out)

**Parámetros:** 4-8 marcos configurables

**Funcionamiento:**

- Reemplaza la página que lleva más tiempo en memoria
- Usa una cola circular (VecDeque en Rust)
- Implementación simple y eficiente

**Ventajas:**

- ✅ Muy simple de implementar
- ✅ Bajo overhead computacional
- ✅ Comportamiento predecible

**Desventajas:**

- ❌ Sufre de la anomalía de Belady (más marcos → más fallos)
- ❌ Puede reemplazar páginas frecuentemente usadas

**Cuándo usar:** Sistemas con patrones de acceso secuenciales, recursos limitados

---

#### 2.2 LRU (Least Recently Used)

**Parámetros:** 4-8 marcos configurables

**Funcionamiento:**

- Reemplaza la página menos recientemente usada
- Mantiene timestamp de último acceso por página
- Aproxima comportamiento óptimo

**Ventajas:**

- ✅ Mejor tasa de aciertos que FIFO
- ✅ No sufre anomalía de Belady
- ✅ Se adapta a patrones de acceso

**Desventajas:**

- ❌ Mayor overhead (mantener timestamps)
- ❌ Más complejo de implementar

**Cuándo usar:** Sistemas generales, workloads con localidad temporal

---

#### 2.3 Working Set (Algoritmo Avanzado)

**Parámetros:** Ventana Δ = 10 referencias

**Funcionamiento:**

- Mantiene conjunto de páginas usadas en últimas Δ referencias
- Ajusta número de marcos según working set del proceso
- Previene thrashing

**Ventajas:**

- ✅ Se adapta dinámicamente al comportamiento
- ✅ Previene thrashing efectivamente
- ✅ Mejor uso de memoria global

**Desventajas:**

- ❌ Complejidad de implementación alta
- ❌ Overhead de mantener ventana temporal

**Cuándo usar:** Sistemas multiprogramados, prevención de thrashing, cargas variables

---

### 3. PLANIFICACIÓN DE DISCO

#### 3.1 FCFS (First Come First Served)

**Parámetros:** 200 cilindros (0-199)

**Funcionamiento:**

- Atiende solicitudes en orden de llegada
- No hay reordenamiento ni optimización
- Equivalente a FIFO para disco

**Ventajas:**

- ✅ Justo: todas las solicitudes se atienden en orden
- ✅ No hay inanición
- ✅ Implementación trivial

**Desventajas:**

- ❌ Movimiento del cabezal puede ser muy largo
- ❌ Bajo rendimiento con cargas aleatorias

**Cuándo usar:** Cargas ligeras, accesos secuenciales, simplicidad prioritaria

---

#### 3.2 SCAN (Algoritmo del Ascensor)

**Parámetros:** 200 cilindros, dirección inicial configurable

**Funcionamiento:**

- El cabezal se mueve en una dirección hasta el extremo
- Atiende todas las solicitudes en el camino
- Al llegar al extremo, invierte dirección
- Comportamiento similar a un ascensor

**Ventajas:**

- ✅ Reduce movimiento total vs FCFS
- ✅ Evita inanición
- ✅ Rendimiento predecible

**Desventajas:**

- ❌ Solicitudes en el centro se atienden más rápido
- ❌ Solicitudes en extremos esperan más

**Cuándo usar:** Sistemas con alta carga de I/O, SSDs/HDDs modernos

---

### 4. SINCRONIZACIÓN

#### 4.1 Semáforos (Dijkstra)

**Implementación:** Semáforo contador con cola de espera

**Operaciones:**

- `wait()`: Decrementa contador; bloquea si ≤ 0
- `signal()`: Incrementa contador; despierta proceso bloqueado

**Usos:**

- Exclusión mutua (mutex)
- Sincronización productor-consumidor
- Control de recursos limitados

---

#### 4.2 Problema Productor-Consumidor

**Parámetros:** Buffer de 5 slots

**Semáforos usados:**

- `mutex = 1` - Exclusión mutua del buffer
- `empty = 5` - Slots vacíos disponibles
- `full = 0` - Slots ocupados disponibles

**Invariantes:**

- Buffer nunca excede tamaño máximo
- No se producen race conditions
- No hay deadlock

---

#### 4.3 Cena de los Filósofos

**Parámetros:** 5 filósofos, 5 tenedores

**Solución implementada:** Orden asimétrico

- Filósofos 0-3: toman tenedor izquierdo, luego derecho
- Filósofo 4: toma tenedor derecho, luego izquierdo
- Rompe la dependencia circular → previene deadlock

**Invariantes:**

- A lo más 4 filósofos pueden intentar comer simultáneamente
- No se produce deadlock
- Todos los filósofos eventualmente comen (no inanición)

---

## 📊 COMPARATIVA DE ALGORITMOS

### Planificación de CPU: RR vs SJF

| Métrica                   | Round Robin           | SJF              |
| ------------------------- | --------------------- | ---------------- |
| Tiempo de respuesta       | Excelente             | Malo (peor caso) |
| Tiempo de espera promedio | Bueno                 | Óptimo           |
| Fairness                  | Excelente             | Malo (inanición) |
| Overhead                  | Alto (context switch) | Bajo             |
| Uso típico                | Interactivo           | Batch            |

---

### Memoria: FIFO vs LRU vs Working Set

| Métrica          | FIFO   | LRU   | Working Set |
| ---------------- | ------ | ----- | ----------- |
| Complejidad      | Baja   | Media | Alta        |
| Tasa de aciertos | Baja   | Alta  | Muy Alta    |
| Overhead         | Mínimo | Medio | Alto        |
| Anomalía Belady  | Sí     | No    | No          |
| Adaptabilidad    | Nula   | Buena | Excelente   |

---

### Disco: FCFS vs SCAN

| Métrica           | FCFS           | SCAN            |
| ----------------- | -------------- | --------------- |
| Movimiento total  | Alto           | Bajo            |
| Inanición         | No             | No              |
| Varianza latencia | Alta           | Baja            |
| Complejidad       | Trivial        | Baja            |
| Uso típico        | Cargas ligeras | Sistemas reales |

---

## 🎯 DECISIONES DE DISEÑO

### ¿Por qué estos algoritmos?

1. **Round Robin:** Requisito explícito del PDF 2
2. **SJF:** Requisito de "al menos 2 algoritmos" de CPU
3. **FIFO memoria:** Requisito explícito del PDF 2 (o LRU)
4. **LRU:** Requisito de "al menos 2 algoritmos" de memoria
5. **Working Set:** Requisito de "algoritmo avanzado" (PFF o Working Set)
6. **FCFS disco:** Base simple para comparar
7. **SCAN:** Requisito de "SSTF o SCAN"

### Configuración de parámetros

- **Quantum RR:** 4 unidades (balance overhead/respuesta)
- **Marcos memoria:** 4-8 (configurable, simula restricción real)
- **Cilindros disco:** 200 (estándar en ejemplos académicos)
- **Buffer productor-consumidor:** 5 slots (típico)
- **Ventana Working Set:** 10 referencias (académicamente común)

---

## 📈 MÉTRICAS A MEDIR

### CPU Scheduling

- Tiempo de espera promedio
- Tiempo de retorno promedio
- Tiempo de respuesta
- Throughput (procesos/unidad de tiempo)

### Memoria

- Fallos de página totales
- Tasa de aciertos (hit rate %)
- PFF (Page Fault Frequency)
- Gráfico: Fallos vs. Número de marcos

### Disco

- Movimiento total del cabezal (cilindros recorridos)
- Tiempo de acceso promedio
- Varianza de latencias
- Gráfico: Recorrido del cabezal

---

**Conclusión:** Esta selección de algoritmos cumple TODOS los requisitos de los 3 PDFs y permite comparativas significativas entre estrategias básicas, intermedias y avanzadas.
