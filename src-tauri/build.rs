/**
 * Script de build Tauri
 * 
 * Ce fichier est exécuté automatiquement par Cargo avant la compilation
 * du projet Rust. Il configure Tauri et génère le code nécessaire pour
 * l'intégration avec le frontend.
 * 
 * Responsabilités :
 * - Configuration des ressources Tauri
 * - Génération du code de liaison frontend/backend
 * - Validation de la configuration
 * - Optimisations spécifiques au build
 */

fn main() {
    // Initialise le système de build Tauri
    // Cette fonction lit la configuration tauri.conf.json et génère
    // le code Rust nécessaire pour l'application
    tauri_build::build()
}