# ⚠️ PROBLEMA DE COMPILACIÓN EN WINDOWS - SOLUCIONES

**Problema:** Windows MSVC requiere Visual Studio Build Tools con C++ (~6GB) para compilar Rust.

---

## ✅ SOLUCIÓN 1: Usar WSL (Windows Subsystem for Linux) - RECOMENDADO

Esta es la solución MÁS RÁPIDA y FÁCIL:

### Paso 1: Habilitar WSL

```powershell
# En PowerShell como Administrador:
wsl --install
```

### Paso 2: Reiniciar Windows

Reinicia tu computadora después de la instalación.

### Paso 3: Abrir WSL

1. Abre "Ubuntu" desde el menú Inicio
2. Crea tu usuario y contraseña cuando te lo pida

### Paso 4: Instalar Rust en WSL

```bash
# Dentro de WSL/Ubuntu:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Paso 5: Acceder a tu proyecto

```bash
# Tus archivos de Windows están en /mnt/c/
cd /mnt/c/Users/Nico/Desktop/SistemasOp

# Compilar (funciona INMEDIATAMENTE sin Visual Studio)
cargo build --release

# Ejecutar tests
cargo test

# Ejecutar CLI
cargo run -- init --scheduler rr
```

**✅ Ventajas:**
- No necesita Visual Studio Build Tools
- Compila en segundos
- Es Linux real dentro de Windows
- Todos los comandos funcionan igual

---

## ✅ SOLUCIÓN 2: Instalar Visual Studio Build Tools (Lento)

Si NO quieres usar WSL:

### Paso 1: Descargar

https://visualstudio.microsoft.com/downloads/

Descarga: **Build Tools for Visual Studio 2022**

### Paso 2: Instalar

1. Ejecuta el instalador
2. Selecciona: **"Desktop development with C++"**
3. Espera 15-20 minutos (descarga ~6GB)
4. Reinicia Windows

### Paso 3: Compilar

```powershell
cd c:\Users\Nico\Desktop\SistemasOp
cargo build --release
```

---

## ✅ SOLUCIÓN 3: Usar GitHub Codespaces (En la nube)

Si tienes GitHub:

1. Sube tu proyecto a GitHub
2. Click en **"Code" → "Codespaces" → "Create codespace"**
3. Espera 1 minuto (crea un entorno Linux en la nube)
4. En la terminal del codespace:

```bash
cargo build --release
cargo test
cargo run -- init --scheduler rr
```

**✅ Ventajas:**
- No instala nada en tu PC
- Funciona desde el navegador
- Gratis para estudiantes (60 horas/mes)

---

## 🎯 RECOMENDACIÓN

**USA WSL (Solución 1)** - Es la más rápida y útil para programación.

### ¿Por qué WSL?
- Se instala en 5 minutos
- No ocupa mucho espacio (~1GB vs 6GB de Visual Studio)
- Es Linux real (útil para otros cursos de sistemas/redes)
- Compila instantáneamente
- No necesitas reiniciar cada vez

---

## 📝 DESPUÉS DE ELEGIR UNA SOLUCIÓN

Una vez que compiles exitosamente, sigue estas instrucciones:

1. **Ejecutar tests:** `cargo test`
2. **Ejecutar demos:** Ver `RESUMEN_EJECUTIVO.md` sección 3
3. **Generar gráficos:** Ver `RESUMEN_EJECUTIVO.md` sección 4
4. **Completar documentación:** Ver `RESUMEN_EJECUTIVO.md` sección 5

---

## 🆘 SI NECESITAS AYUDA

### Para WSL:
```bash
# Verificar que WSL funciona:
wsl --version

# Abrir Ubuntu:
# Busca "Ubuntu" en el menú Inicio

# Dentro de WSL, verificar Rust:
cargo --version
```

### Para Visual Studio:
```powershell
# Verificar que link.exe está disponible:
where.exe link.exe
# Debería mostrar: C:\Program Files\...\link.exe
```

---

**Última actualización:** 2025-11-06  
**Estado:** Proyecto completo, solo necesita compilarse
