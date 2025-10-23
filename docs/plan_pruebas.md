```markdown
# Plan de pruebas (resumen)

Tipos de pruebas

- Unitarias: scheduler (FIFO/RR), memoria (alloc/free/translation), ipc (semaforos), io/disk.
- Integración: crear múltiples procesos con I/O y verificar estados finales.
- E2E: escenarios predefinidos: CPU-bound, I/O-bound, mezcla.
- Reports automatizados: `cargo test` + export de logs JSON.

Ejemplo de caso unitario

- Test: FIFO_dispatch_order
  - Setup: P1(5), P2(3), P3(2), scheduler=FIFO
  - Esperado: orden P1, P2, P3 y timestamps consistentes.

Automatización

- `cargo test`
- Incluir pruebas en `tests/` y usar fixtures en `tests/data/`.
```
