# 🚀 Guide de Build de Production - MacGyver

Ce guide détaille le processus de build optimisé pour créer un exécutable MacGyver performant et de taille minimale.

## 📋 Prérequis

### Outils Requis
- **Node.js** >= 18.0.0
- **Rust** >= 1.70.0
- **Tauri CLI** >= 1.5.0
- **PowerShell** (pour Windows)

### Installation des Outils
```bash
# Installer Tauri CLI
cargo install tauri-cli

# Vérifier les versions
node --version
cargo --version
tauri --version
```

## 🏗️ Processus de Build

### 1. Build Rapide (Développement)
```bash
npm run tauri:build
```

### 2. Build Optimisé (Production)
```bash
npm run tauri:build:prod
```

### 3. Build Complet avec Script Automatisé
```powershell
.\scripts\build-production.ps1
```

## ⚡ Optimisations Appliquées

### Frontend (Vite + SvelteKit)
- ✅ **Minification Terser** avec compression agressive
- ✅ **Tree Shaking** pour supprimer le code inutilisé
- ✅ **Code Splitting** pour optimiser le cache
- ✅ **Compression Gzip/Brotli** des assets
- ✅ **Suppression des console.log** en production
- ✅ **Optimisation CSS** avec minification

### Backend (Rust + Tauri)
- ✅ **LTO (Link Time Optimization)** complète
- ✅ **Optimisation de taille** avec `opt-level = "z"`
- ✅ **Strip des symboles** de debug
- ✅ **Panic = abort** pour réduire la taille
- ✅ **Codegen-units = 1** pour une meilleure optimisation
- ✅ **Target-cpu = native** pour les performances

### Bundle Windows
- ✅ **Compression LZMA** pour l'installateur
- ✅ **Métadonnées complètes** pour l'application
- ✅ **Thème sombre** par défaut
- ✅ **Icônes optimisées** pour toutes les tailles

## 📊 Résultats Attendus

### Tailles Typiques
- **Exécutable** : ~15-25 MB (optimisé)
- **Installateur MSI** : ~20-30 MB
- **Installateur NSIS** : ~18-28 MB

### Performances
- **Temps de démarrage** : < 2 secondes
- **Utilisation mémoire** : ~50-100 MB
- **Taille sur disque** : ~30-50 MB

## 🔧 Configuration Avancée

### Variables d'Environnement
```bash
# Build de production
NODE_ENV=production

# Optimisations Rust
CARGO_PROFILE_RELEASE_LTO=fat
CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
CARGO_PROFILE_RELEASE_PANIC=abort
CARGO_PROFILE_RELEASE_STRIP=symbols
```

### Profils de Build Personnalisés

#### Profil Ultra-Optimisé (Taille)
```toml
[profile.release-size]
inherits = "release"
opt-level = "z"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"
```

#### Profil Performance
```toml
[profile.release-perf]
inherits = "release"
opt-level = 3
lto = "thin"
codegen-units = 16
```

## 🐛 Dépannage

### Erreurs Communes

#### "Tauri CLI not found"
```bash
cargo install tauri-cli --force
```

#### "Build failed: linker error"
```bash
# Installer les outils de build Windows
# Via Visual Studio Installer ou Build Tools
```

#### "Out of memory during build"
```bash
# Augmenter la mémoire virtuelle
# Ou utiliser un profil moins agressif
```

### Optimisation de la Vitesse de Build
```bash
# Utiliser sccache pour le cache de compilation
cargo install sccache
set RUSTC_WRAPPER=sccache

# Build incrémental en développement
cargo build --release --incremental
```

## 📁 Structure des Fichiers de Sortie

```
src-tauri/target/x86_64-pc-windows-msvc/release/bundle/
├── msi/
│   └── MacGyver_1.0.0_x64_fr-FR.msi     # Installateur MSI
├── nsis/
│   └── MacGyver_1.0.0_x64-setup.exe     # Installateur NSIS
└── exe/
    └── macgyver.exe                      # Exécutable portable
```

## 🚀 Distribution

### Signature de Code (Optionnel)
```bash
# Configurer le certificat dans tauri.conf.json
"certificateThumbprint": "YOUR_CERT_THUMBPRINT"
```

### Tests de l'Exécutable
```bash
# Tester l'exécutable
.\src-tauri\target\x86_64-pc-windows-msvc\release\macgyver.exe

# Tester l'installateur
.\src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\MacGyver_1.0.0_x64_fr-FR.msi
```

## 📈 Métriques de Performance

Pour analyser les performances du build :

```bash
# Analyser la taille du bundle
npm run analyze

# Profiler le build Rust
cargo build --release --timings

# Analyser les dépendances
cargo tree --duplicates
```

---

**🎉 Votre application MacGyver est maintenant optimisée pour la production !**