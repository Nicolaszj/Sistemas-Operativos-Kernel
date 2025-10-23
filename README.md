```markdown
# kernel-sim (Rust) — Simulación de núcleo simplificado

Resumen

- Simulación funcional de un kernel simplificado: planificación de procesos, gestión de memoria, sincronización/IPC, IO/disk simulados.
- Interfaz CLI.
- Implementado en Rust (recomendado por seguridad de memoria y concurrencia).

Requisitos

- Rust (rustup), cargo (edición 2021)
- Recomendado: `rustup default stable`

Estructura propuesta

- kernel-sim/
  - Cargo.toml
  - src/
    - main.rs
    - lib.rs
    - cli/
    - kernel.rs
    - process.rs
    - scheduler.rs
    - modules/
      - cpu/
      - mem/
      - io/
      - disk/
      - ipc/
  - docs/
    - alcance.md
    - arquitectura.md
    - plan_pruebas.md
    - backlog.md
  - tests/
  - .gitignore

Comandos útiles

- Compilar: cargo build
- Ejecutar: cargo run -- run --config examples/config.toml
- Tests: cargo test
```
