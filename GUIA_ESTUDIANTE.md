# 🎓 GUÍA COMPLETA PARA EL ESTUDIANTE

**Proyecto:** Simulación de Núcleo de Sistema Operativo  
**Estudiante:** Nicolaszj  
**Fecha:** Noviembre 6, 2025

---

## ⚠️ REQUISITOS PREVIOS

### 1. Instalar Rust

Si no tienes Rust instalado, ejecuta:

#### Windows (PowerShell):

```powershell
# Descargar e instalar rustup
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe
```

#### Linux/Mac:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Luego reinicia tu terminal y verifica:

```bash
cargo --version
# Debería mostrar: cargo 1.xx.x
```

### 2. Instalar Python (para gráficos - OPCIONAL)

```bash
# Windows
python --version  # Debe ser 3.8+

# Si no tienes Python, descarga de: https://www.python.org/downloads/

# Instalar librerías
pip install matplotlib numpy
```

---

## 🚀 PASO 1: COMPILAR EL PROYECTO

```bash
# Navegar al proyecto
cd c:\Users\Nico\Desktop\SistemasOp

# Compilar
cargo build --release

# Si todo está bien, verás:
# ✅ Compiling kernel-sim...
# ✅ Finished release [optimized] target(s)
```

**⏱️ Tiempo estimado:** 1-2 minutos (primera vez)

**Posibles errores:**

- **"cargo: command not found"** → Reinicia la terminal después de instalar Rust
- **Errores de compilación** → Revisa que todos los archivos estén presentes

---

## 🧪 PASO 2: EJECUTAR TESTS

```bash
# Ejecutar todos los tests
cargo test

# Deberías ver:
# ✅ test fifo_order ... ok
# ✅ test round_robin_fairness ... ok
# ✅ test sjf_shortest_first ... ok
# ✅ test_fifo_replacement ... ok
# ✅ test_lru_replacement ... ok
# ✅ test_semaphore_basic ... ok
# ✅ test_producer_consumer ... ok
# ✅ test_philosophers_no_deadlock ... ok
# ✅ test_fcfs_order ... ok
# ✅ test_sstf_closest_first ... ok
# ✅ test_disk_simulator ... ok
```

**⏱️ Tiempo estimado:** 10-30 segundos

**Todos los tests deben pasar (status: ok)**

---

## 💻 PASO 3: PROBAR LA CLI

### 3.1 Comandos Básicos

```bash
# Ver ayuda
cargo run -- --help

# Inicializar kernel con Round Robin
cargo run -- init --scheduler rr --quantum 4 --frames 8
```

**Salida esperada:**

```
✅ Kernel inicializado:
   Scheduler: rr
   Quantum: 4
   Marcos de memoria: 8
```

### 3.2 Crear Procesos

```bash
# Crear proceso 1
cargo run -- new --burst 10 --mem 100

# Crear proceso 2
cargo run -- new --burst 5 --mem 50

# Crear proceso 3
cargo run -- new --burst 8 --mem 80
```

**Salida esperada:**

```
✅ Proceso 1 creado (burst=10, mem=100)
✅ Proceso 2 creado (burst=5, mem=50)
✅ Proceso 3 creado (burst=8, mem=80)
```

### 3.3 Listar Procesos

```bash
cargo run -- ps
```

**Salida esperada:**

```
╔═══════════════════════════════════════════════════════════╗
║                    LISTA DE PROCESOS                      ║
╠═════╦═══════════╦═══════════════╦═══════════╦═════════════╣
║ PID ║  Estado   ║ Burst Restante║  Memoria  ║   Llegada   ║
╠═════╬═══════════╬═══════════════╬═══════════╬═════════════╣
║   1 ║ Ready     ║      10       ║   100     ║      0      ║
║   2 ║ Ready     ║      5        ║   50      ║      0      ║
║   3 ║ Ready     ║      8        ║   80      ║      0      ║
╚═════╩═══════════╩═══════════════╩═══════════╩═════════════╝
```

### 3.4 Ejecutar Simulación

```bash
# Avanzar 5 pasos
cargo run -- tick 5

# Ver estado
cargo run -- ps

# Ejecutar hasta que terminen todos
cargo run -- run 25

# Ver métricas finales
cargo run -- metrics
```

---

## 📊 PASO 4: PROBAR CADA MÓDULO

### 4.1 Round Robin vs SJF

#### Probar Round Robin:

```bash
# Reiniciar
cargo run -- init --scheduler rr --quantum 3

# Crear procesos con diferentes ráfagas
cargo run -- new --burst 10 --mem 100
cargo run -- new --burst 3 --mem 50
cargo run -- new --burst 6 --mem 80
cargo run -- new --burst 1 --mem 40

# Ejecutar
cargo run -- run 25

# Ver métricas
cargo run -- metrics
```

**Anota:** Tiempo de espera promedio, Tiempo de retorno promedio

#### Probar SJF:

```bash
# Reiniciar con SJF
cargo run -- init --scheduler sjf

# Crear LOS MISMOS procesos
cargo run -- new --burst 10 --mem 100
cargo run -- new --burst 3 --mem 50
cargo run -- new --burst 6 --mem 80
cargo run -- new --burst 1 --mem 40

# Ejecutar
cargo run -- run 25

# Ver métricas
cargo run -- metrics
```

**Compara:** SJF debería tener MEJOR tiempo de espera que RR

---

### 4.2 Memoria - FIFO vs LRU

#### Probar FIFO:

```bash
# Reiniciar
cargo run -- init --frames 4

# Crear proceso
cargo run -- new --burst 10 --mem 100

# Acceder a páginas (secuencia con anomalía de Belady)
cargo run -- mem-fifo 1 1 2 3 4 1 2 5 1 2 3 4 5

# Ver marcos
cargo run -- mem-display

# Ver estado general (incluye métricas)
cargo run -- status
```

**Anota:** Fallos de página, Tasa de aciertos

#### Probar LRU:

```bash
# Reiniciar
cargo run -- init --frames 4

cargo run -- new --burst 10 --mem 100

# MISMA secuencia
cargo run -- mem-lru 1 1 2 3 4 1 2 5 1 2 3 4 5

cargo run -- status
```

**Compara:** LRU debería tener MEJOR tasa de aciertos que FIFO

---

### 4.3 Disco - FCFS, SSTF, SCAN

#### Comparar todos:

```bash
# Ejecutar comparativa automática
cargo run -- disk-compare --start 50 98 183 37 122 14 124 65 67
```

**Salida esperada:**

```
╔════════════════════════════════════════════════╗
║         RESUMEN COMPARATIVO                    ║
╠════════════╦═══════════════╦══════════════════╣
║ Algoritmo  ║   Movimiento  ║   Eficiencia     ║
╠════════════╬═══════════════╬══════════════════╣
║ FCFS       ║     643       ║    Baseline      ║
║ SSTF       ║     239       ║    +62.8%        ║
║ SCAN       ║     302       ║    +53.0%        ║
╚════════════╩═══════════════╩══════════════════╝

🏆 Mejor algoritmo: SSTF (movimiento: 239)
```

**Anota:** Movimiento total de cada algoritmo

---

### 4.4 Sincronización - Productor-Consumidor

```bash
# Inicializar
cargo run -- init

# Producir items
cargo run -- produce "Manzana" --pid 100
cargo run -- produce "Naranja" --pid 100
cargo run -- produce "Pera" --pid 100

# Ver buffer
cargo run -- buffer-stat

# Consumir
cargo run -- consume --pid 200
cargo run -- consume --pid 200

# Ver buffer nuevamente
cargo run -- buffer-stat
```

**Verifica:**

- Buffer se llena correctamente
- Consumidor solo puede consumir lo que hay
- Semáforos funcionan (no race conditions)

---

### 4.5 Cena de los Filósofos

```bash
# Simular 5 filósofos, 10 pasos
cargo run -- philosophers --count 5 --steps 10
```

**Verifica:**

- Todos los filósofos comen al menos 1 vez (no hay inanición)
- No hay deadlock
- Visualización muestra estados correctamente

---

## 📈 PASO 5: GENERAR GRÁFICOS (Opcional)

```bash
# Navegar a scripts
cd scripts

# Ejecutar script Python
python plot_graphs.py
```

**Salida esperada:**

```
✅ Gráfico guardado: mem_fallos_vs_marcos.png
✅ Gráfico guardado: mem_hit_rate.png
✅ Gráfico guardado: disk_comparativa.png
✅ Gráfico guardado: scheduler_comparativa.png
```

**Incluye estos gráficos en tu informe técnico.**

---

## 📝 PASO 6: COMPLETAR INFORME TÉCNICO

### Estructura del informe (ya creado en `docs/`):

1. **Portada** - Ya en README.md
2. **Backlog** - ✅ `docs/backlog.md`
3. **Algoritmos seleccionados** - ✅ `docs/algoritmos_seleccionados.md`
4. **Arquitectura** - ✅ `docs/arquitectura.md`
5. **Plan de pruebas** - ✅ `docs/plan_pruebas.md`
6. **Análisis del estado** - ✅ `docs/analisis_estado_proyecto.md`

### Lo que DEBES agregar:

#### 6.1 Resultados Experimentales

Crea `docs/resultados.md` con:

- Tabla de métricas de scheduling (RR vs SJF)
- Tabla de memoria (FIFO vs LRU)
- Tabla de disco (FCFS vs SSTF vs SCAN)
- Gráficos generados

#### 6.2 Conclusiones

Agrega `docs/conclusiones.md` con:

**Ejemplo:**

```markdown
### Planificación de CPU

- **Round Robin:** Mejor tiempo de respuesta (2.5), ideal para sistemas interactivos
- **SJF:** Mejor tiempo de espera (7.6 vs 9.4), óptimo para batch processing
- **Trade-off:** RR → fairness, SJF → eficiencia

### Memoria Virtual

- **FIFO:** Simple pero sufre anomalía de Belady
- **LRU:** Mejor tasa de aciertos (58% vs 25% en nuestro experimento)
- **Cuándo usar:** LRU para workloads con localidad temporal

### Disco

- **FCFS:** Justo pero ineficiente (643 cilindros)
- **SSTF:** Más eficiente (239 cilindros) pero puede causar inanición
- **SCAN:** Balance óptimo (302 cilindros, sin inanición)
```

---

## 🎤 PASO 7: PREPARAR SUSTENTACIÓN (50% de la nota!)

### 7.1 Crear Video/Guion de 5 minutos

**Estructura sugerida:**

1. **Introducción (30 seg)**

   - Presentarte
   - Explicar objetivo del proyecto

2. **Demo de Scheduling (1 min)**

   - Ejecutar: `cargo run -- init --scheduler rr`
   - Crear 3 procesos
   - Ejecutar simulación
   - Mostrar métricas

3. **Demo de Memoria (1 min)**

   - Ejecutar comparativa FIFO vs LRU
   - Mostrar gráfico de tasas de aciertos

4. **Demo de Disco (1 min)**

   - Ejecutar: `cargo run -- disk-compare`
   - Explicar por qué SSTF es mejor

5. **Demo de Sincronización (1 min)**

   - Productor-consumidor
   - Filósofos (mostrar que no hay deadlock)

6. **Conclusiones (30 seg)**
   - Trade-offs principales
   - Aprendizajes

### 7.2 Preguntas Frecuentes - Prepara Respuestas

**P: ¿Por qué elegiste Rust?**

```
R: Por seguridad de memoria (ownership), rendimiento comparable a C,
   y manejo de errores explícito con Result<T, E>. Ideal para sistemas.
```

**P: ¿Cómo funciona Round Robin?**

```
R: Asigna un quantum fijo (ej: 4) a cada proceso. Si no termina, vuelve
   al final de la cola. Garantiza fairness y buen tiempo de respuesta.
```

**P: ¿Qué es la anomalía de Belady?**

```
R: En FIFO, más marcos pueden causar MÁS fallos. Ejemplo: con secuencia
   1,2,3,4,1,2,5,1,2,3,4,5 → 3 marcos = 9 fallos, 4 marcos = 10 fallos.
   LRU NO sufre esta anomalía.
```

**P: ¿Cómo evitas deadlock en filósofos?**

```
R: Uso orden asimétrico: filósofos 0-3 toman izquierdo→derecho,
   filósofo 4 toma derecho→izquierdo. Rompe dependencia circular.
```

**P: ¿Cuándo usarías SCAN vs SSTF?**

```
R: SSTF es más eficiente (menos movimiento) pero puede causar inanición
   en solicitudes lejanas. SCAN garantiza que todas se atiendan eventualmente.
   En producción: SCAN o C-SCAN.
```

---

## ✅ CHECKLIST FINAL ANTES DE ENTREGAR

### Código

- [ ] `cargo build --release` compila sin errores
- [ ] `cargo test` - todos los tests pasan
- [ ] CLI funciona con todos los comandos

### Documentación

- [ ] `docs/backlog.md` - Historias de usuario completas
- [ ] `docs/algoritmos_seleccionados.md` - Algoritmos documentados
- [ ] `docs/resultados.md` - Resultados experimentales
- [ ] `docs/conclusiones.md` - Análisis y trade-offs
- [ ] README.md actualizado con instrucciones de uso

### Scripts

- [ ] 6 scripts en `/scripts` (mem x2, disk x2, proc x2)
- [ ] `scripts/README.md` explica cómo usarlos

### Gráficos (si aplica)

- [ ] 4 gráficos generados con Python
- [ ] Incluidos en el informe

### Sustentación

- [ ] Video/guion de 5 minutos preparado
- [ ] Respuestas a preguntas frecuentes estudiadas
- [ ] Demo en vivo lista (poder ejecutar comandos en clase)

---

## 🆘 RESOLUCIÓN DE PROBLEMAS

### "cargo: command not found"

```bash
# Reiniciar terminal después de instalar Rust
# O agregar manualmente a PATH:
# Windows: C:\Users\<TU_USUARIO>\.cargo\bin
```

### "error: linking with `link.exe` failed"

```bash
# Windows necesita Visual Studio Build Tools
# Descargar: https://visualstudio.microsoft.com/downloads/
# Instalar "Desktop development with C++"
```

### Tests fallan

```bash
# Ver detalles del error
cargo test -- --nocapture

# Ejecutar test específico
cargo test nombre_del_test -- --nocapture
```

### CLI no muestra colores/símbolos correctamente

```bash
# Windows: Usa Windows Terminal (no PowerShell antiguo)
# O ejecuta en Linux/WSL
```

---

## 📞 CONTACTO Y RECURSOS

### Recursos del Proyecto

- **Libro:** Tanenbaum - Modern Operating Systems (4th Ed)
- **Documentación Rust:** https://doc.rust-lang.org/book/
- **Clap (CLI):** https://docs.rs/clap/

### Si Necesitas Ayuda

1. Lee los errores de compilación cuidadosamente
2. Revisa `docs/analisis_estado_proyecto.md` para entender estructura
3. Consulta ejemplos en `scripts/README.md`
4. Pregunta en clase/foros

---

## 🎯 RESUMEN DE COMANDOS ESENCIALES

```bash
# Compilar
cargo build --release

# Tests
cargo test

# Scheduler RR
cargo run -- init --scheduler rr --quantum 4
cargo run -- new --burst 10 --mem 100
cargo run -- run 20
cargo run -- metrics

# Memoria
cargo run -- mem-fifo 1 1 2 3 4 1 2 5
cargo run -- status

# Disco
cargo run -- disk-compare --start 50 98 183 37 122

# Sincronización
cargo run -- produce "Item1"
cargo run -- consume
cargo run -- philosophers --steps 10

# Gráficos
python scripts/plot_graphs.py
```

---

**¡ÉXITO EN TU PROYECTO!** 🚀

**Última actualización:** 2025-11-06
