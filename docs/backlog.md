# Backlog del Proyecto - Kernel Simulation

**Proyecto:** Simulación de Núcleo de Sistema Operativo  
**Estudiante:** Nicolaszj  
**Fecha:** Noviembre 6, 2025

---

## 📋 COMPONENTE 1: PLANIFICACIÓN DE PROCESOS (CPU Scheduling)

### Historia de Usuario 1.1: Implementar Round Robin

**Como** usuario del sistema  
**Quiero** que los procesos se ejecuten con Round Robin  
**Para** garantizar equidad y evitar que procesos largos bloqueen a los cortos

**Criterios de Aceptación:**

1. El scheduler debe asignar un quantum fijo (3-5 unidades) a cada proceso
2. Si un proceso no termina en su quantum, debe volver al final de la cola
3. Si un proceso termina antes del quantum, debe liberarse inmediatamente
4. El sistema debe calcular y mostrar tiempo de espera promedio
5. El sistema debe calcular y mostrar tiempo de retorno promedio

**Prioridad:** ALTA  
**Estimación:** 2-3 horas

---

### Historia de Usuario 1.2: Implementar SJF (Shortest Job First)

**Como** administrador del sistema  
**Quiero** un planificador SJF no expropiativo  
**Para** minimizar el tiempo de espera promedio cuando conozco las ráfagas

**Criterios de Aceptación:**

1. El scheduler debe ordenar procesos por cpu_burst de menor a mayor
2. Una vez iniciado un proceso, debe ejecutarse hasta completarse
3. El sistema debe documentar si usa burst conocido o estimado
4. Debe compararse con Round Robin mostrando diferencias en métricas

**Prioridad:** MEDIA  
**Estimación:** 1-2 horas

---

## 📋 COMPONENTE 2: GESTIÓN DE MEMORIA

### Historia de Usuario 2.1: Implementar Paginación con FIFO

**Como** sistema operativo  
**Quiero** gestionar memoria virtual usando paginación FIFO  
**Para** permitir que procesos usen más memoria de la físicamente disponible

**Criterios de Aceptación:**

1. Cada proceso debe tener su propia tabla de páginas
2. Al acceder a una página no cargada, debe ocurrir un fallo de página
3. Si no hay marcos libres, debe reemplazarse la página más antigua (FIFO)
4. El sistema debe contar fallos totales y calcular tasa de aciertos
5. Debe visualizarse el estado de los marcos en consola (ASCII)

**Prioridad:** ALTA  
**Estimación:** 3-4 horas

---

### Historia de Usuario 2.2: Implementar Paginación con LRU

**Como** sistema operativo  
**Quiero** un algoritmo LRU (Least Recently Used) para memoria  
**Para** mejorar la tasa de aciertos comparado con FIFO

**Criterios de Aceptación:**

1. Debe mantener timestamp o contador de accesos por página
2. Al reemplazar, debe elegirse la página menos recientemente usada
3. Debe exportarse datos CSV para comparar con FIFO
4. Debe generarse gráfico: fallos vs. número de marcos (FIFO vs LRU)

**Prioridad:** MEDIA  
**Estimación:** 2-3 horas

---

### Historia de Usuario 2.3: Implementar Algoritmo Avanzado (PFF o Working Set)

**Como** sistema operativo avanzado  
**Quiero** un tercer algoritmo de memoria (PFF o Working Set)  
**Para** adaptarme dinámicamente al comportamiento de los procesos

**Criterios de Aceptación:**

1. Si se elige PFF: debe ajustar marcos según frecuencia de fallos
2. Si se elige Working Set: debe mantener ventana de tiempo de páginas activas
3. Debe documentarse claramente el diseño y parámetros elegidos
4. Debe compararse con FIFO y LRU en el informe

**Prioridad:** BAJA (valor agregado)  
**Estimación:** 3-4 horas

---

## 📋 COMPONENTE 3: ENTRADA/SALIDA Y DISCO

### Historia de Usuario 3.1: Implementar Planificación de Disco FCFS

**Como** sistema de archivos  
**Quiero** un planificador de disco FCFS (First Come First Served)  
**Para** atender solicitudes en orden de llegada

**Criterios de Aceptación:**

1. Debe mantener una cola de solicitudes de cilindros
2. Debe atender solicitudes en orden FIFO
3. Debe calcular movimiento total del cabezal
4. Debe mostrar visualización del recorrido (línea de cilindros)

**Prioridad:** MEDIA  
**Estimación:** 1-2 horas

---

### Historia de Usuario 3.2: Implementar SSTF o SCAN

**Como** sistema de archivos optimizado  
**Quiero** un segundo algoritmo de disco (SSTF o SCAN)  
**Para** reducir el movimiento total del cabezal

**Criterios de Aceptación:**

1. Si SSTF: debe elegir la solicitud más cercana al cabezal actual
2. Si SCAN: debe barrer en una dirección hasta el final, luego invertir
3. Debe compararse con FCFS mostrando reducción de movimiento
4. Debe generarse gráfico comparativo de ambos algoritmos

**Prioridad:** MEDIA  
**Estimación:** 2-3 horas

---

## 📋 COMPONENTE 4: SINCRONIZACIÓN (IPC)

### Historia de Usuario 4.1: Implementar Semáforos Básicos

**Como** mecanismo de sincronización  
**Quiero** semáforos con operaciones wait() y signal()  
**Para** coordinar el acceso a recursos compartidos

**Criterios de Aceptación:**

1. `wait()` debe decrementar contador; si ≤0, bloquear proceso
2. `signal()` debe incrementar contador y despertar proceso bloqueado
3. Debe garantizarse exclusión mutua en operaciones críticas
4. Debe mantenerse una cola de procesos bloqueados por semáforo

**Prioridad:** ALTA  
**Estimación:** 1-2 horas

---

### Historia de Usuario 4.2: Resolver Problema Productor-Consumidor

**Como** sistema con procesos concurrentes  
**Quiero** implementar el problema productor-consumidor  
**Para** demostrar sincronización con semáforos

**Criterios de Aceptación:**

1. Debe existir un buffer compartido de tamaño fijo
2. Productor debe bloquearse si buffer está lleno
3. Consumidor debe bloquearse si buffer está vacío
4. Debe usarse 3 semáforos: mutex, empty, full
5. Comandos CLI: `produce <item>`, `consume`, `stat`

**Prioridad:** ALTA  
**Estimación:** 2-3 horas

---

### Historia de Usuario 4.3: Resolver Cena de los Filósofos

**Como** sistema avanzado de sincronización  
**Quiero** implementar la cena de los filósofos  
**Para** demostrar prevención de deadlock

**Criterios de Aceptación:**

1. Debe haber 5 filósofos y 5 tenedores (semáforos)
2. Cada filósofo debe pensar, tomar tenedores, comer, soltar tenedores
3. Debe evitarse deadlock (solución: orden de tenedores, mayordomo, etc.)
4. Debe visualizarse estado de cada filósofo en tiempo real
5. Comando CLI: `philosophers <pasos>`

**Prioridad:** MEDIA  
**Estimación:** 2-3 horas

---

## 📋 COMPONENTE 5: INTERFAZ CLI

### Historia de Usuario 5.1: Comandos Básicos de Procesos

**Como** usuario del sistema  
**Quiero** comandos para crear y gestionar procesos  
**Para** interactuar con el kernel desde la terminal

**Criterios de Aceptación:**

1. `new <burst> <mem>` - Crear proceso y agregarlo al scheduler activo
2. `ps` - Listar todos los procesos con: PID, estado, burst restante
3. `kill <pid>` - Terminar un proceso específico inmediatamente
4. Debe mostrarse confirmación de cada operación exitosa
5. Debe manejarse errores (PID inválido, parámetros incorrectos)

**Prioridad:** ALTA  
**Estimación:** 1-2 horas

---

### Historia de Usuario 5.2: Control de Simulación

**Como** usuario del sistema  
**Quiero** controlar el avance de la simulación  
**Para** ejecutar paso a paso o en bloques

**Criterios de Aceptación:**

1. `tick [n]` - Avanzar n pasos de tiempo (default: 1)
2. `run <n>` - Ejecutar n pasos completos de simulación
3. Debe mostrarse el tiempo simulado actual
4. Debe mostrarse qué proceso está en CPU en cada tick
5. Debe actualizarse el estado de procesos (Running, Ready, Blocked, Terminated)

**Prioridad:** ALTA  
**Estimación:** 1-2 horas

---

### Historia de Usuario 5.3: Visualización del Sistema

**Como** usuario del sistema  
**Quiero** ver el estado de memoria, disco y procesos  
**Para** entender qué está ocurriendo internamente

**Criterios de Aceptación:**

1. Comando `status` - Mostrar estado general del sistema
2. Vista de marcos de memoria (tabla ASCII, indicar hits/fallos)
3. Vista de disco (línea de cilindros, posición cabezal, cola)
4. Panel de procesos (scheduler activo, proceso en CPU, cola)
5. Uso opcional de colores para mejor visualización

**Prioridad:** MEDIA  
**Estimación:** 2-3 horas

---

## 📋 COMPONENTE 6: PRUEBAS Y VALIDACIÓN

### Historia de Usuario 6.1: Tests Unitarios

**Como** desarrollador  
**Quiero** tests unitarios para cada módulo  
**Para** garantizar correcto funcionamiento individual

**Criterios de Aceptación:**

1. Test Round Robin: verificar orden circular y fairness
2. Test SJF: verificar orden por burst más corto
3. Test Paginación FIFO: verificar orden de reemplazo
4. Test Paginación LRU: verificar página menos usada
5. Test Semáforos: verificar wait/signal bloquean/despiertan
6. Todos los tests deben pasar con `cargo test`

**Prioridad:** ALTA  
**Estimación:** 2-3 horas

---

### Historia de Usuario 6.2: Tests de Integración

**Como** desarrollador  
**Quiero** tests que validen integración entre módulos  
**Para** asegurar que el sistema funciona como un todo

**Criterios de Aceptación:**

1. Test E2E: crear procesos → ejecutar → verificar terminación correcta
2. Test: procesos con I/O → verificar bloqueo y reanudación
3. Test: múltiples procesos compitiendo por memoria
4. Test: productor-consumidor sin race conditions
5. Resultados deben exportarse a JSON para análisis

**Prioridad:** MEDIA  
**Estimación:** 2-3 horas

---

## 📋 COMPONENTE 7: DOCUMENTACIÓN Y REPRODUCIBILIDAD

### Historia de Usuario 7.1: Scripts de Reproducción

**Como** evaluador del proyecto  
**Quiero** scripts para reproducir experimentos  
**Para** validar resultados sin configuración manual

**Criterios de Aceptación:**

1. `scripts/mem_test1.txt` - Traza de accesos a memoria con parámetros
2. `scripts/disk_fcfs.txt` - Secuencia de cilindros para FCFS
3. `scripts/proc_scenario1.txt` - Escenario de procesos con llegadas/ráfagas
4. Cada script debe tener formato claro y documentado
5. Debe existir un README en `/scripts` explicando formato

**Prioridad:** MEDIA  
**Estimación:** 1-2 horas

---

### Historia de Usuario 7.2: Informe Técnico Completo

**Como** estudiante  
**Quiero** un informe técnico profesional  
**Para** documentar diseño, implementación y resultados

**Criterios de Aceptación:**

1. Secciones completas: Memoria, Disco, Sincronización, Interfaz
2. Gráficos comparativos generados desde datos reales
3. Análisis de trade-offs de cada algoritmo
4. Conclusiones sobre cuándo usar cada implementación
5. Formato profesional con portada, índice, referencias

**Prioridad:** ALTA (20% de la nota)  
**Estimación:** 3-4 horas

---

## 📊 RESUMEN DE PRIORIDADES

### 🔴 ALTA (Parte 2 - Urgente)

- Historia 1.1: Round Robin
- Historia 2.1: Paginación FIFO
- Historia 4.1: Semáforos
- Historia 4.2: Productor-Consumidor
- Historia 5.1: Comandos básicos CLI
- Historia 5.2: Control de simulación
- Historia 6.1: Tests unitarios
- Historia 7.2: Informe técnico

**Total estimado:** 15-20 horas

### 🟡 MEDIA (Parte 3 - Necesario)

- Historia 1.2: SJF
- Historia 2.2: Paginación LRU
- Historia 3.1: Disco FCFS
- Historia 3.2: Disco SSTF/SCAN
- Historia 4.3: Filósofos
- Historia 5.3: Visualización
- Historia 6.2: Tests integración
- Historia 7.1: Scripts reproducción

**Total estimado:** 13-18 horas

### 🟢 BAJA (Valor agregado)

- Historia 2.3: Algoritmo avanzado (PFF/Working Set)

**Total estimado:** 3-4 horas

---

**TOTAL GENERAL:** 31-42 horas de desarrollo

---

## 📝 NOTAS

- Este backlog cubre TODOS los requisitos de los 3 PDFs
- Cada historia tiene ≥2 criterios de aceptación (requisito cumplido)
- Las prioridades están alineadas con las fechas de entrega
- Los tiempos son estimaciones; pueden variar según experiencia

**Fecha de creación:** 2025-11-06  
**Última actualización:** 2025-11-06
