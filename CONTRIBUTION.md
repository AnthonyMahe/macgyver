# 🤝 Guide de Contribution - MacGyver

Merci de votre intérêt pour contribuer à MacGyver ! Ce guide vous aidera à démarrer.

## 📋 Table des Matières

- [Code de Conduite](#code-de-conduite)
- [Comment Contribuer](#comment-contribuer)
- [Configuration de Développement](#configuration-de-développement)
- [Standards de Code](#standards-de-code)
- [Processus de Pull Request](#processus-de-pull-request)

## 🤝 Code de Conduite

Ce projet adhère à notre [Code de Conduite](CODE_DE_CONDUITE.md). En participant, vous vous engagez à respecter ce code.

## 🚀 Comment Contribuer

### Types de Contributions Bienvenues

- 🐛 **Corrections de bugs**
- ✨ **Nouvelles fonctionnalités**
- 📚 **Amélioration de la documentation**
- 🎨 **Améliorations UI/UX**
- 🔧 **Optimisations de performance**
- 🌍 **Améliorations de traduction française**

### Signaler un Bug

1. Vérifiez que le bug n'a pas déjà été signalé
2. Créez une issue avec le modèle "Rapport de Bug"
3. Incluez :
   - Description détaillée
   - Étapes pour reproduire
   - Comportement attendu vs actuel
   - Captures d'écran si applicable
   - Informations système (Windows, version, etc.)

### Proposer une Fonctionnalité

1. Créez une issue avec le modèle "Demande de Fonctionnalité"
2. Décrivez clairement :
   - Le problème que cela résout
   - La solution proposée
   - Les alternatives considérées

## 🛠️ Configuration de Développement

### Prérequis

- **Node.js** >= 18.0.0
- **Rust** >= 1.70.0
- **Git**
- **Visual Studio Build Tools** (pour Windows)

### Installation

```bash
# Cloner le repository
git clone https://github.com/votre-nom-utilisateur/macgyver.git
cd macgyver

# Installer les dépendances
npm install

# Lancer en mode développement
npm run tauri:dev
```

### Structure du Projet

```
macgyver/
├── src/                    # Frontend SvelteKit
│   ├── lib/               # Composants réutilisables
│   ├── routes/            # Pages de l'application
│   └── app.html           # Modèle HTML principal
├── src-tauri/             # Backend Rust
│   ├── src/               # Code source Rust
│   ├── Cargo.toml         # Configuration Rust
│   └── tauri.conf.json    # Configuration Tauri
└── docs/                  # Documentation
```

## 📏 Standards de Code

### Frontend (TypeScript/Svelte)

- **Formatage** : Prettier avec configuration du projet
- **Linting** : ESLint avec règles TypeScript
- **Conventions** :
  - Variables : `camelCase`
  - Composants : `PascalCase`
  - Fichiers : `kebab-case`
- **Langue** : Tous les textes et commentaires en français

### Backend (Rust)

- **Formatage** : `cargo fmt`
- **Linting** : `cargo clippy`
- **Conventions** :
  - Variables/fonctions : `snake_case`
  - Types/structs : `PascalCase`
  - Constantes : `SCREAMING_SNAKE_CASE`
- **Langue** : Commentaires et messages d'erreur en français

### Messages de Commit

Utilisez des messages de commit en français :

```
type(portée): description

feat(pomodoro): ajouter fonctionnalité de pause
fix(pdf): corriger fuite mémoire lors du traitement
docs(readme): mettre à jour instructions d'installation
```

Types acceptés : `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## 🔄 Processus de Pull Request

### Avant de Soumettre

1. **Fork** le repository
2. **Créez une branche** : `git checkout -b fonctionnalite/ma-fonctionnalite`
3. **Testez** vos changements :
   ```bash
   npm run check        # Vérification TypeScript
   npm run build        # Build de production
   cargo test           # Tests Rust
   cargo clippy         # Linting Rust
   ```

### Soumettre la PR

1. **Poussez** votre branche : `git push origin fonctionnalite/ma-fonctionnalite`
2. **Créez la Pull Request** sur GitHub
3. **Remplissez le modèle** de PR en français
4. **Liez les issues** concernées

### Modèle de PR

```markdown
## 📝 Description
Brève description des changements

## 🔗 Issues Liées
Corrige #123

## 🧪 Tests
- [ ] Tests unitaires ajoutés/mis à jour
- [ ] Tests manuels effectués
- [ ] Build de production testé

## 📸 Captures d'écran
(Si applicable)

## ✅ Liste de Vérification
- [ ] Code formaté et linté
- [ ] Documentation mise à jour
- [ ] Changements testés
- [ ] Messages de commit suivent les conventions
- [ ] Tous les textes sont en français
```

## 🧪 Tests

### Lancer les Tests

```bash
# Tests frontend
npm run test

# Tests backend
cargo test

# Tests d'intégration
npm run test:integration
```

### Écrire des Tests

- **Frontend** : Utilisez Vitest pour les tests unitaires
- **Backend** : Tests Rust standard avec `#[cfg(test)]`
- **E2E** : Tests avec Playwright (à venir)
- **Langue** : Noms de tests et assertions en français

## 📚 Documentation

- Documentez les nouvelles fonctionnalités en français
- Mettez à jour le README si nécessaire
- Ajoutez des commentaires en français pour le code complexe
- Utilisez JSDoc en français pour les fonctions TypeScript

## 🎉 Reconnaissance

Tous les contributeurs seront ajoutés au fichier `CONTRIBUTEURS.md` et mentionnés dans les versions.

## 📞 Besoin d'Aide ?

- 💬 **Discussions** : Utilisez les GitHub Discussions (en français)
- 📧 **Email** : contact@macgyver-app.com
- 🐛 **Issues** : Pour les bugs et demandes de fonctionnalités

## 🌟 Conseils pour Contribuer

### Bonnes Pratiques

- 📝 **Écrivez en français** : Tous les textes, commentaires et documentation
- 🧪 **Testez localement** : Assurez-vous que tout fonctionne avant de soumettre
- 📚 **Documentez** : Expliquez vos changements clairement
- 🤝 **Soyez respectueux** : Suivez notre code de conduite
- 💡 **Restez simple** : Préférez les solutions simples et claires

### Domaines d'Aide Recherchés

- 🍅 **Fonctionnalités Pomodoro** : Améliorations du timer
- 📝 **Gestion des Notes** : Nouvelles fonctionnalités d'édition
- ✅ **TodoList** : Fonctionnalités avancées de gestion des tâches
- 🎨 **Interface Utilisateur** : Améliorations visuelles
- 🔧 **Performance** : Optimisations diverses
- 📱 **Accessibilité** : Améliorer l'accessibilité de l'interface

---

**Merci de contribuer à MacGyver ! 🚀**