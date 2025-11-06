# Script de instalación automática de WSL + Rust
# Ejecutar como Administrador

Write-Host "==================================" -ForegroundColor Cyan
Write-Host "INSTALACIÓN AUTOMÁTICA DE WSL" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Verificar si se está ejecutando como administrador
$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
$isAdmin = $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "❌ ERROR: Este script debe ejecutarse como Administrador" -ForegroundColor Red
    Write-Host ""
    Write-Host "Pasos:" -ForegroundColor Yellow
    Write-Host "1. Cierra esta ventana" -ForegroundColor Yellow
    Write-Host "2. Click derecho en PowerShell" -ForegroundColor Yellow
    Write-Host "3. Selecciona 'Ejecutar como administrador'" -ForegroundColor Yellow
    Write-Host "4. Ejecuta: .\install_wsl.ps1" -ForegroundColor Yellow
    pause
    exit 1
}

Write-Host "✅ Ejecutando como Administrador" -ForegroundColor Green
Write-Host ""

# Paso 1: Instalar WSL
Write-Host "📥 Instalando WSL (Windows Subsystem for Linux)..." -ForegroundColor Cyan
try {
    wsl --install -d Ubuntu
    Write-Host "✅ WSL instalado exitosamente" -ForegroundColor Green
} catch {
    Write-Host "❌ Error instalando WSL: $_" -ForegroundColor Red
    pause
    exit 1
}

Write-Host ""
Write-Host "==================================" -ForegroundColor Cyan
Write-Host "INSTALACIÓN COMPLETA" -ForegroundColor Green
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "PRÓXIMOS PASOS:" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. REINICIA Windows (obligatorio)" -ForegroundColor Yellow
Write-Host ""
Write-Host "2. Después del reinicio, abre 'Ubuntu' desde el menú Inicio" -ForegroundColor Yellow
Write-Host ""
Write-Host "3. Crea tu usuario y contraseña cuando te lo pida" -ForegroundColor Yellow
Write-Host ""
Write-Host "4. Dentro de Ubuntu, ejecuta estos comandos:" -ForegroundColor Yellow
Write-Host ""
Write-Host "   # Instalar Rust" -ForegroundColor Cyan
Write-Host "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" -ForegroundColor White
Write-Host "   source `$HOME/.cargo/env" -ForegroundColor White
Write-Host ""
Write-Host "   # Ir a tu proyecto" -ForegroundColor Cyan
Write-Host "   cd /mnt/c/Users/Nico/Desktop/SistemasOp" -ForegroundColor White
Write-Host ""
Write-Host "   # Compilar" -ForegroundColor Cyan
Write-Host "   cargo build --release" -ForegroundColor White
Write-Host ""
Write-Host "   # Ejecutar tests" -ForegroundColor Cyan
Write-Host "   cargo test" -ForegroundColor White
Write-Host ""
Write-Host "   # Probar CLI" -ForegroundColor Cyan
Write-Host "   cargo run -- init --scheduler rr" -ForegroundColor White
Write-Host ""
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Presiona Enter para reiniciar ahora..." -ForegroundColor Yellow
pause

# Reiniciar
Restart-Computer -Force
