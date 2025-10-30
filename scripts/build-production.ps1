# Script PowerShell pour build de production optimisé
# Usage: .\scripts\build-production.ps1

Write-Host "🚀 Début du build de production MacGyver..." -ForegroundColor Green

# Vérification des prérequis
Write-Host "🔍 Vérification des prérequis..." -ForegroundColor Yellow

# Vérifier Node.js
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Node.js n'est pas installé!" -ForegroundColor Red
    exit 1
}

# Vérifier Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust n'est pas installé!" -ForegroundColor Red
    exit 1
}

# Vérifier Tauri CLI
if (-not (Get-Command tauri -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Tauri CLI n'est pas installé!" -ForegroundColor Red
    Write-Host "Installez avec: cargo install tauri-cli" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ Tous les prérequis sont installés" -ForegroundColor Green

# Nettoyage des builds précédents
Write-Host "🧹 Nettoyage des builds précédents..." -ForegroundColor Yellow
if (Test-Path "build") { Remove-Item -Recurse -Force "build" }
if (Test-Path "dist") { Remove-Item -Recurse -Force "dist" }
if (Test-Path "src-tauri/target") { Remove-Item -Recurse -Force "src-tauri/target" }

# Installation des dépendances
Write-Host "📦 Installation des dépendances..." -ForegroundColor Yellow
npm ci --production=false

# Build du frontend avec optimisations
Write-Host "🏗️ Build du frontend avec optimisations..." -ForegroundColor Yellow
$env:NODE_ENV = "production"
npm run build:prod

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Erreur lors du build frontend!" -ForegroundColor Red
    exit 1
}

# Build de l'application Tauri
Write-Host "🦀 Build de l'application Tauri..." -ForegroundColor Yellow
$env:TAURI_PRIVATE_KEY = ""
$env:TAURI_KEY_PASSWORD = ""

# Build avec profil release optimisé
tauri build --config src-tauri/tauri.conf.json --target x86_64-pc-windows-msvc

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Erreur lors du build Tauri!" -ForegroundColor Red
    exit 1
}

# Affichage des résultats
Write-Host "✅ Build terminé avec succès!" -ForegroundColor Green
Write-Host "📁 Fichiers générés dans:" -ForegroundColor Cyan

$bundlePath = "src-tauri/target/x86_64-pc-windows-msvc/release/bundle"
if (Test-Path $bundlePath) {
    Get-ChildItem $bundlePath -Recurse -File | Where-Object { $_.Extension -in @('.exe', '.msi') } | ForEach-Object {
        $size = [math]::Round($_.Length / 1MB, 2)
        Write-Host "  📄 $($_.Name) - ${size} MB" -ForegroundColor White
        Write-Host "     $($_.FullName)" -ForegroundColor Gray
    }
}

Write-Host "🎉 MacGyver est prêt pour la distribution!" -ForegroundColor Green