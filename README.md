# 🛠️ MacGyver

[![Licence: MIT](https://img.shields.io/badge/Licence-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-1.5-blue.svg)](https://tauri.app/)
[![SvelteKit](https://img.shields.io/badge/SvelteKit-2.0-orange.svg)](https://kit.svelte.dev/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-red.svg)](https://www.rust-lang.org/)

> Suite d'outils de productivité moderne et open source

Application desktop légère combinant **Tauri**, **SvelteKit** et **Rust** pour une expérience utilisateur fluide et performante.

## ✨ Fonctionnalités

- 🍅 **Timer Pomodoro** - Technique de concentration par cycles de 25 minutes
- 📝 **Notes Rapides** - Prise de notes instantanée et organisée
- ✅ **TodoList** - Gestion simple et efficace de vos tâches
- 🖼️ **Studio d'Images** - Conversion et manipulation d'images
- 📄 **Manipulateur PDF** - Fusion et division de documents PDF

## 🚀 Installation

### Téléchargement

Téléchargez la dernière version depuis les [Releases](https://github.com/AnthonyMahe/macgyver/releases) :

- **Windows** : `macgyver_x.x.x_x64_fr-FR.msi`
- **Linux** : `macgyver_x.x.x_amd64.AppImage`

### Installation rapide

**Windows :** Double-cliquez sur le fichier `.msi`

**Linux :**
```bash
chmod +x macgyver_*.AppImage
./macgyver_*.AppImage
```

## 🛠️ Développement

### Prérequis

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) (Windows)

### Démarrage rapide

```bash
# Installation des dépendances
npm install

# Lancement en mode développement
npm run tauri:dev

# Build de production
npm run tauri:build
```

## 🎨 Technologies

- **Frontend :** SvelteKit + TypeScript
- **Backend :** Rust + Tauri
- **Build :** Vite + Cargo
- **Taille :** ~4.6 MB optimisé

## 🤝 Contribution

Les contributions sont les bienvenues ! Consultez [CONTRIBUTION.md](CONTRIBUTION.md) pour commencer.

## 📄 Licence

Ce projet est sous licence [MIT](LICENCE).

---

**Fait avec ❤️ et Rust**