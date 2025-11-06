# ✅ CHECKLIST COMPLETO - Proyecto Final Sistemas Operativos

**Estudiante:** Nicolaszj  
**Fecha:** Noviembre 6, 2025  
**Repositorio:** Sistemas-Operativos-Kernel

---

## 📚 PARTE 1 - Fundamentos, Alcance y Arquitectura

### 📄 Documentación Base

- [x] **Portada** en README.md
- [x] **Objetivo general** documentado
- [ ] **Componentes y algoritmos** seleccionados (falta especificar en detalle)
- [x] **Stack seleccionado:** Rust + CLI
- [x] **Diagrama de arquitectura** inicial (bloques y contratos)
- [ ] **Backlog con historias de usuario** (≥2 criterios por componente) ⚠️ FALTANTE
- [x] **Estructura de repositorio** creada
- [x] **Borrador de informe técnico:**
  - [x] Alcance (`docs/alcance.md`)
  - [x] Supuestos (`docs/alcance.md`)
  - [x] Arquitectura (`docs/arquitectura.md`)
  - [x] Plan de pruebas (`docs/plan_pruebas.md`)

### 🎯 Algoritmos a Seleccionar y Documentar

- [ ] **CPU Scheduling:** Round Robin + SJF
- [ ] **Memoria Virtual:** FIFO + LRU + (PFF o Working Set)
- [ ] **Planificación de Disco:** FCFS + (SSTF o SCAN)
- [ ] **Sincronización:** Productor-Consumidor + Filósofos

**Estado Parte 1:** 🟡 70% - Falta backlog y especificar algoritmos

---

## 💻 PARTE 2 - Implementación Core

### 1. Planificador Round Robin

- [ ] Crear `RoundRobinScheduler` que implemente trait `Scheduler`
- [ ] Agregar campo `quantum` configurable
- [ ] Modificar `Process` para incluir `remaining_burst`
- [ ] Implementar rotación cuando se agota quantum
- [ ] Test: `test_round_robin_fairness()`
- [ ] Test: Medir tiempos de espera y retorno
- [ ] Documentar diseño en `/docs/scheduler.md`

### 2. CLI Extendida

- [x] ~~`run --config <file>`~~ (ya existe)
- [ ] `new <burst> <mem>` - Crear proceso y agregarlo
- [ ] `ps` - Listar procesos con estado
- [ ] `tick [n]` - Avanzar n pasos (default: 1)
- [ ] `kill <pid>` - Terminar proceso
- [ ] `run <n>` - Ejecutar n pasos completos
- [ ] Crear struct `KernelState` para mantener estado global

### 3. Gestión de Memoria - Paginación

- [ ] Crear `src/modules/mem/page_table.rs`
  - [ ] Struct `PageTable` por proceso
  - [ ] Mapeo página → marco
- [ ] Crear `src/modules/mem/frame_manager.rs`
  - [ ] Struct `FrameManager` con lista de marcos
  - [ ] Implementar algoritmo **FIFO** de reemplazo
- [ ] **Métricas:**
  - [ ] Contador de fallos de página totales
  - [ ] Contador de aciertos (hits)
  - [ ] Calcular tasa de aciertos (hit rate %)
  - [ ] PFF puntual (opcional)
- [ ] **Visualización:**
  - [ ] Función `display_frames()` - Mostrar marcos en consola
  - [ ] Indicar hits/fallos con símbolos o colores
- [ ] Tests de paginación

### 4. Sincronización - Semáforos

- [ ] Implementar métodos en `Semaphore`:
  - [ ] `wait()` - Decrementar, bloquear si ≤ 0
  - [ ] `signal()` - Incrementar, despertar proceso
- [ ] Crear `src/modules/ipc/producer_consumer.rs`
  - [ ] Buffer compartido simulado
  - [ ] Semáforo `mutex` para exclusión mutua
  - [ ] Semáforo `empty` para slots vacíos
  - [ ] Semáforo `full` para slots ocupados
- [ ] **Comandos CLI adicionales:**
  - [ ] `produce <item>` - Agregar al buffer
  - [ ] `consume` - Extraer del buffer
  - [ ] `stat` - Mostrar estado buffer y semáforos
- [ ] Crear script de demo: `scripts/demo_sync.txt`

### 5. Pruebas y Métricas

- [ ] Test unitario: Round Robin orden correcto
- [ ] Test unitario: Round Robin fairness
- [ ] Test unitario: Paginación FIFO
- [ ] Test integración: Crear procesos → ejecutar → verificar estados
- [ ] Métricas de scheduling:
  - [ ] Tiempo de espera promedio
  - [ ] Tiempo de retorno promedio
  - [ ] Tiempo de respuesta

### 6. Documentación Parte 2

- [ ] Documentar diseño del planificador RR
- [ ] Documentar diseño de paginación FIFO
- [ ] Documentar diseño de sincronización
- [ ] Documentar invariantes del productor-consumidor

**Estado Parte 2:** 🔴 10% - Solo estructura base

---

## 🚀 PARTE 3 - Integración y Componentes Avanzados

### 1. Segundo Algoritmo de CPU - SJF

- [ ] Crear `SjfScheduler` (Shortest Job First)
- [ ] Implementar ordenamiento por `cpu_burst`
- [ ] Versión no expropiativa
- [ ] Test: Verificar orden por burst más corto
- [ ] Documentar supuestos (¿burst conocido o estimado?)

### 2. Segundo Algoritmo de Memoria - LRU

- [ ] Implementar **LRU** (Least Recently Used)
- [ ] Mantener timestamps o lista de accesos
- [ ] Comparar métricas vs FIFO
- [ ] **Gráficas:**
  - [ ] Crear script Python/Jupyter: `scripts/plot_memory.py`
  - [ ] Gráfico: Fallos de página vs. número de marcos
  - [ ] Comparativa: FIFO vs LRU
  - [ ] Exportar datos a CSV

### 3. Tercer Algoritmo de Memoria (Avanzado)

- [ ] Elegir uno: **PFF** (Page Fault Frequency) o **Working Set**
- [ ] Implementar lógica de ventana de tiempo
- [ ] Documentar diseño
- [ ] Métricas comparativas con FIFO y LRU

### 4. Asignador en Heap (Opcional - Valor Agregado)

- [ ] Elegir: Buddy System o Segregated Free Lists
- [ ] Implementar `alloc()` y `free()`
- [ ] Medir fragmentación interna/externa
- [ ] Medir latencia de alloc/free

### 5. Planificación de Disco

- [ ] Crear `src/modules/disk/scheduler.rs`
- [ ] Trait `DiskScheduler`
- [ ] Implementar **FCFS** (First Come First Served)
- [ ] Implementar **SSTF** (Shortest Seek Time First) o **SCAN**
- [ ] **Métricas:**
  - [ ] Movimiento total del cabezal
  - [ ] Tiempo promedio de acceso
- [ ] **Gráfico:**
  - [ ] Visualización de movimiento del cabezal
  - [ ] Comparativa: FCFS vs SSTF/SCAN
  - [ ] Crear script: `scripts/plot_disk.py`

### 6. Cena de los Filósofos

- [ ] Crear `src/modules/ipc/philosophers.rs`
- [ ] 5 filósofos, 5 tenedores (semáforos)
- [ ] Implementar lógica para evitar deadlock
- [ ] Solución: Tenedor izquierdo/derecho con orden
- [ ] Comando CLI: `philosophers <steps>`
- [ ] Visualización del estado de cada filósofo

### 7. Visualización Avanzada

- [ ] **Vista de marcos de memoria:**
  - [ ] Tabla ASCII con marcos
  - [ ] Colores: verde=hit, rojo=fallo
  - [ ] Mostrar proceso dueño de cada marco
- [ ] **Vista de disco:**
  - [ ] Línea de cilindros (0-199)
  - [ ] Posición actual del cabezal
  - [ ] Cola de solicitudes pendientes
- [ ] **Panel de procesos:**
  - [ ] Tabla con PID, estado, burst restante
  - [ ] Indicar qué proceso está en CPU
  - [ ] Mostrar algoritmo activo (RR/SJF)

### 8. Scripts de Reproducción

- [ ] **Memoria:** `scripts/mem_test1.txt`
  - [ ] Trazas de acceso a páginas
  - [ ] Parámetros: número de marcos
  - [ ] Ejemplo: `4,8,1,2,5,3,4,8,1,5` (secuencia de páginas)
- [ ] **Memoria:** `scripts/mem_test2.txt` (caso diferente)
- [ ] **Disco:** `scripts/disk_fcfs.txt`
  - [ ] Secuencia de cilindros solicitados
  - [ ] Ejemplo: `98,183,37,122,14,124,65,67`
- [ ] **Disco:** `scripts/disk_scan.txt` (caso diferente)
- [ ] **Procesos:** `scripts/proc_scenario1.txt`
  - [ ] Llegadas: tiempo, burst, memoria
  - [ ] Ejemplo: `0,5,100 | 2,3,50 | 4,8,80`
- [ ] **Procesos:** `scripts/proc_scenario2.txt` (caso diferente)

### 9. Informe Técnico Final

- [ ] **Portada completa**
- [ ] **Sección: Memoria Virtual**
  - [ ] Explicar FIFO, LRU y algoritmo avanzado elegido
  - [ ] Métricas de cada uno
  - [ ] Gráficos comparativos (fallos vs marcos)
  - [ ] Conclusión: ¿Cuándo usar cada uno?
- [ ] **Sección: Asignador en Heap**
  - [ ] Diseño elegido (Buddy/Segregated)
  - [ ] Mediciones de fragmentación
  - [ ] Latencia de alloc/free
- [ ] **Sección: Planificación de Disco**
  - [ ] Comparativa FCFS vs SSTF/SCAN
  - [ ] Gráfico de recorrido del cabezal
  - [ ] Tiempos de acceso
  - [ ] Conclusión: Trade-offs
- [ ] **Sección: Sincronización**
  - [ ] Diseño de semáforos
  - [ ] Invariantes del productor-consumidor
  - [ ] Solución de filósofos (cómo evita deadlock)
  - [ ] Resultados de pruebas
- [ ] **Sección: Diseño de Interfaz**
  - [ ] Comandos CLI documentados
  - [ ] Capturas de pantalla de ejecución
  - [ ] Flujo de uso
  - [ ] (Si hay GUI): capturas adicionales
- [ ] **Sección: Conclusiones**
  - [ ] Trade-offs de cada algoritmo
  - [ ] Recomendaciones de uso
  - [ ] Lecciones aprendidas

### 10. Sustentación (50% de la nota)

- [ ] **Preparar guion de 5 minutos**
- [ ] **Video corto demostrando:**
  - [ ] Ejecución de Round Robin
  - [ ] Paginación con visualización
  - [ ] Productor-consumidor
  - [ ] Filósofos
  - [ ] Planificación de disco
- [ ] **Estudiar el código** para responder preguntas
- [ ] **Preparar respuestas** para preguntas comunes:
  - [ ] ¿Por qué elegiste Rust?
  - [ ] ¿Cómo funciona Round Robin?
  - [ ] ¿Qué es un fallo de página?
  - [ ] ¿Cómo evitas deadlock en filósofos?
  - [ ] ¿Cuándo usarías LRU vs FIFO?

**Estado Parte 3:** 🔴 5% - Solo estructura mínima

---

## 📊 RESUMEN DE PROGRESO GLOBAL

| Componente              | Requerido    | Implementado | Estado |
| ----------------------- | ------------ | ------------ | ------ |
| **Documentación**       | 5 docs       | 3/5          | 🟡 60% |
| **Backlog**             | 1 doc        | 0/1          | 🔴 0%  |
| **CPU Scheduling**      | 2 algoritmos | 0.5/2        | 🔴 25% |
| **Memoria Virtual**     | 3 algoritmos | 0/3          | 🔴 0%  |
| **Planificación Disco** | 2 algoritmos | 0/2          | 🔴 0%  |
| **Sincronización**      | 2 problemas  | 0/2          | 🔴 0%  |
| **CLI**                 | 7 comandos   | 2/7          | 🔴 29% |
| **Tests**               | 10+ tests    | 1/10         | 🔴 10% |
| **Scripts**             | 6 archivos   | 0/6          | 🔴 0%  |
| **Informe Final**       | 1 doc        | 0/1          | 🔴 0%  |
| **Visualización**       | 3 vistas     | 0/3          | 🔴 0%  |
| **Sustentación**        | Preparada    | No           | 🔴 0%  |

### 🎯 Cobertura Total: **~15%**

---

## 📋 ORDEN DE IMPLEMENTACIÓN RECOMENDADO

### Sprint 1: Parte 2 Completa (8-12 horas)

1. ✅ Round Robin Scheduler (2h)
2. ✅ CLI Extendida (2h)
3. ✅ Paginación FIFO (3h)
4. ✅ Semáforos + Productor-Consumidor (2h)
5. ✅ Tests básicos (1h)
6. ✅ Documentación Parte 2 (1h)

### Sprint 2: Parte 3 Core (6-8 horas)

7. ✅ SJF Scheduler (1h)
8. ✅ LRU + algoritmo avanzado (3h)
9. ✅ Planificación de Disco FCFS + SSTF/SCAN (2h)
10. ✅ Cena de los Filósofos (2h)

### Sprint 3: Visualización y Scripts (4-6 horas)

11. ✅ Scripts de reproducción (1h)
12. ✅ Visualización avanzada (2h)
13. ✅ Gráficos con Python (2h)
14. ✅ Exportar CSVs (1h)

### Sprint 4: Documentación Final (3-4 horas)

15. ✅ Backlog completo (1h)
16. ✅ Informe técnico completo (2h)
17. ✅ Preparar sustentación (1h)

**Tiempo Total Estimado:** 21-30 horas

---

## 🎓 CRITERIOS DE EVALUACIÓN

### Elementos Entregables (50%)

| Criterio                      | Peso | Estado |
| ----------------------------- | ---- | ------ |
| Implementación módulos clave  | 40%  | 🔴 15% |
| Integración entre componentes | 10%  | 🔴 10% |
| Calidad informe técnico       | 20%  | 🟡 40% |
| Pruebas y resultados          | 10%  | 🔴 10% |
| Documentación código          | 10%  | 🟡 50% |
| Valor agregado                | 10%  | 🔴 0%  |

### Sustentación (50%)

- Apropiación del proyecto
- Seguridad en explicaciones
- Claridad en respuestas
- Demostración en vivo

---

## 📝 NOTAS IMPORTANTES

1. **FIFO implementado ≠ Round Robin requerido**

   - Tu FIFO actual sirve como base pero NO cumple requisito
   - FIFO no está en la lista de algoritmos de CPU requeridos

2. **Sustentación vale 50% de la nota total**

   - Debes entender TODO el código
   - Practica explicar cada componente
   - Prepara respuestas a preguntas típicas

3. **Informe técnico es crítico (20% de entregables)**

   - No es solo documentación de código
   - Requiere análisis, gráficos, conclusiones
   - Debe demostrar comprensión de trade-offs

4. **Scripts de reproducción son obligatorios**
   - Profesor debe poder ejecutar experimentos
   - Deben generar resultados consistentes
   - Facilitan validación de implementación

---

**Próximo paso:** Comenzar implementación siguiendo el orden recomendado.

¿Listo para empezar? 🚀
