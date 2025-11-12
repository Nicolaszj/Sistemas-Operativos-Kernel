Escuela de Ciencias Aplicadas e Ingeniería
Ingeniería de Sistemas
Sistemas Operativos
Diego Iván Cruz Ordiéres
Proyecto Final (25%)
Inspira Crea Transforma Vigilada Mineducación 1 / 2
Proyecto Final
Objetivo
Desarrollar una simulación funcional de un núcleo de sistema operativo
simplificado que integre componentes esenciales como gestión de memoria,
planificación de procesos, sincronización, comunicación entre procesos y
mecanismos básicos de entrada/salida.
Componentes del Proyecto
1. Gestión de Procesos
• Creación, suspensión, reanudación y terminación de procesos
simulados.
• Planificador con al menos dos algoritmos de planificación (por
ejemplo: Round Robin y SJF).
2. Memoria Virtual y Paginación
• Simulación de asignación de marcos.
• Implementación de LRU y FIFO.
• Añade uno entre PFF o Working Set (para reflejar “gestión avanzada”).
• Visualización del comportamiento y estadísticas de uso de memoria.
3. Sincronización de Procesos
• Uso de semáforos o mutex para resolver problemas como productorconsumidor o lectores-escritores.
• Implementación del algoritmo de la cena de los filósofos como caso
de estudio.
4. Entrada/Salida y Manejo de Recursos
• Simulación de acceso a dispositivos con colas y prioridades.
• Implementación de una impresora o buffer compartido como recurso.
5. Planificación de Disco
• Simulación de algoritmos de acceso a disco como FCFS
• Incluir SSTF o SCAN.
• Incluir gráfico “movimiento total” por algoritmo en la misma carga.
6. Interfaz de Usuario del Núcleo (CLI)
• Consola o interfaz básica para crear procesos, monitorear memoria,
simular interrupciones, entre otros.
• Vista de marcos de memoria (color por hits/fallos).
• Vista de disco (línea de cilindros + posición del cabezal).
• Panel de procesos/planificador (RR/SJF).
Inspira Crea Transforma Vigilada Mineducación 2 / 2
Entregables
1. Código fuente documentado del sistema.
2. paquete de scripts para reproducir experimentos:
scripts/mem_*.txt (trazas y parámetros de marcos),
scripts/disk_*.txt (secuencias de cilindros),
scripts/proc_*.txt (llegadas y ráfagas).
3. Informe técnico con:
o Memoria virtual: explicar los 2+ algoritmos elegidos (incluye uno
avanzado: PFF o Working Set), sus métricas y gráficos
comparativos.
o Asignador en heap: diseño (Buddy/Segregated), mediciones de
fragmentación y latencia de alloc/free simulados.
o Disco (comparativa): FCFS vs. SSTF/SCAN con recorrido y
tiempos.
o Sincronización: diseño, invari antes y resultados.
o Diseño de interfaz: CLI (comandos) y, si hay GUI, capturas y
flujo.
o Conclusiones: cuándo conviene cada algoritmo y por qué
(trade-offs).
4. Diagrama de módulos y flujos de procesos.
Evaluación
Se establecerá una valoración para los elementos entregables y otra para la
sustentación del proyecto realizado. Se tendrá en cuenta la siguiente
distribución porcentual para la evaluación del proyecto final y los detalles
para las entregas esperadas.
Valoración Total:
• Sustentación: 50%
• Elementos entregables (entregas parciales por cada clase): 50%
Elementos Entregables:
Criterio Ponderación
Implementación de los módulos clave 2.0
Integración entre componentes 0.5
Calidad del informe técnico 1.0
Pruebas, simulación y resultados 0.5
Documentación y estilo del código 0.5
Inspira Crea Transforma Vigilada Mineducación 3 / 2
Criterio Ponderación
Valor agregado o innovación 0.5
En cuanto a la sustentación, se verificará la apropiación, seguridad y claridad
del estudiante en la explicación del proyecto entregado y en las respuestas
otorgadas a las preguntas planteadas.