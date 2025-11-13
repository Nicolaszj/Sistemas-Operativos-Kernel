# Conclusiones y Trade-offs

## Planificación de CPU

- **Round Robin:** Mejor para sistemas interactivos (menor T_respuesta)
- **SJF:** Óptimo para throughput (menor T_espera promedio)
- **Cuándo usar:** RR → interactivo, SJF → batch

## Memoria Virtual

- **FIFO:** Simple pero sufre anomalía de Belady
- **LRU:** Mejor rendimiento, asume localidad temporal
- **Cuándo usar:** LRU para workloads reales, FIFO solo académico

## Disco

- **FCFS:** Justo pero ineficiente
- **SSTF:** Eficiente pero puede causar inanición
- **SCAN:** Balance óptimo (usado en Linux como Deadline Scheduler)

## Sincronización

- **Productor-Consumidor:** 3 semáforos previenen race conditions
- **Filósofos:** Orden asimétrico evita deadlock, mejor que timeout