```markdown
# Alcance y Supuestos (Rust)

Alcance

- Implementar una simulación: planificación de procesos, gestión de memoria (particiones/paginación), sincronización/IPC (semaforos y mensajería simple), IO simulado y disco (lectura/escritura de bloques).
- CLI para crear procesos, seleccionar planificador y ejecutar escenarios.
- Entregables: código fuente, informe técnico (alcance, supuestos, arquitectura), backlog con historias (≥2 criterios por componente), borrador del plan de pruebas (1-2 páginas).

Supuestos

- Simulación lógica (no desarrollo de un kernel real).
- Tiempo simulado (pasos discretos) — no es obligatorio usar threads reales.
- Uso de librerías Rust estándar y crates aprobados (clap, serde, log, etc.).

Restricciones

- Lenguaje: Rust.
- Interfaz CLI obligatoria.
```
