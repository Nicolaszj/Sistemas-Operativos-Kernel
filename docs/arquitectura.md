```markdown
# Arquitectura inicial (Rust) — Bloques y contratos

Módulos principales

- cli: parsing de argumentos y comandos.
- kernel: orquestador principal que coordina scheduler, CPU, memory, IO y IPC.
- scheduler: implementaciones de algoritmos (FIFO, RR).
- process: definición de `Process` y estados.
- modules/\*
  - cpu
  - mem
  - io
  - disk
  - ipc
- logger: uso de `log` + `env_logger`.

APIs simplificadas (ejemplos)

- CLI -> Kernel:
  - kernel::start(config: Option<Path>)
  - kernel::create_process(spec) -> PID
- Scheduler:
  - Scheduler::push(process)
  - Scheduler::next() -> Option<PID>
- Memory:
  - MemoryManager::alloc(pid, size) -> Result<Address>
  - MemoryManager::free(pid)
- IO/Disk:
  - IoManager::enqueue_io(pid, op)
  - Disk::read(block) -> Data

Observabilidad

- Logging con timestamps simulados.
- Exportar traces en JSON para análisis.

Notas

- Diseñar con traits y interfaces para permitir intercambiar algoritmos (por ejemplo, trait `Scheduler` con implementaciones FIFO y RR).
```
