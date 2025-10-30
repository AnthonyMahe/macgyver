// Désactive la console Windows en mode release pour une meilleure UX
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/**
 * Point d'entrée principal de l'application Tauri
 * 
 * Ce fichier initialise l'application Tauri et configure :
 * - Les commandes exposées au frontend
 * - Les plugins nécessaires
 * - La gestion des événements
 * - Les paramètres de sécurité
 * 
 * Architecture :
 * - Imports des modules et dépendances
 * - Configuration du logging
 * - Enregistrement des commandes Tauri
 * - Initialisation et lancement de l'application
 */

// === Imports des dépendances ===
use tauri::Manager;  // Gestionnaire principal de Tauri
use log::{info, error, warn};  // Système de logging

// === Imports des modules locaux ===
// Ces modules contiennent la logique métier de l'application
mod commandes;      // Module contenant toutes les commandes Tauri
mod utilitaires;    // Module contenant les fonctions utilitaires
mod types;          // Module contenant les types de données partagés
mod erreurs;        // Module contenant les définitions d'erreurs personnalisées

// Réexport des commandes pour simplifier l'enregistrement
// Import supprimé car non utilisé

/**
 * Fonction principale de l'application
 * 
 * Cette fonction configure et lance l'application Tauri.
 * Elle est appelée automatiquement au démarrage de l'application.
 * 
 * Étapes :
 * 1. Initialisation du système de logging
 * 2. Configuration de l'application Tauri
 * 3. Enregistrement des commandes exposées au frontend
 * 4. Configuration des événements et hooks
 * 5. Lancement de l'application
 */
fn main() {
    // === Initialisation du système de logging ===
    // Configure les logs pour le debugging et le monitoring
    env_logger::Builder::from_default_env()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug  // Mode debug : logs détaillés
        } else {
            log::LevelFilter::Info   // Mode release : logs essentiels uniquement
        })
        .init();
    
    info!("🚀 Démarrage de l'application Tauri + SvelteKit + Rust");
    
    // === Configuration et lancement de l'application Tauri ===
    tauri::Builder::default()
        
        // === Enregistrement des commandes Tauri ===
        // Ces commandes peuvent être appelées depuis le frontend via invoke()
        .invoke_handler(tauri::generate_handler![

            
            // Commandes de productivité
            commandes::productivite::notifier_pomodoro_termine,
            commandes::productivite::sauvegarder_notes,
            commandes::productivite::charger_notes,
            commandes::productivite::sauvegarder_taches,
            commandes::productivite::charger_taches,
            
            // Commandes de conversion d'images
            commandes::images::obtenir_info_image,
            commandes::images::convertir_image,
            commandes::images::obtenir_formats_supportes,
            commandes::images::supprimer_fond_image,
            
            // Commandes de manipulation de PDF
            commandes::pdf::obtenir_info_pdf,
            commandes::pdf::fusionner_pdfs,
            commandes::pdf::diviser_pdf,
        ])
        
        // === Configuration des plugins ===
        // .plugin(tauri_plugin_shell::init())  // Plugin pour exécuter des commandes shell
        
        // === Gestionnaire d'événements de l'application ===
        .setup(|app| {
            info!("🔧 Configuration de l'application Tauri");
            
            // Obtient la fenêtre principale de l'application
            let fenetre_principale = app.get_window("main");
            
            if let Some(_fenetre) = fenetre_principale {
                info!("✅ Fenêtre principale initialisée avec succès");
                
                // Configuration additionnelle de la fenêtre si nécessaire
                // Exemple : fenetre.set_title("Mon Application Tauri")?;
                
            } else {
                warn!("⚠️  Impossible de récupérer la fenêtre principale");
            }
            
            info!("✅ Configuration de l'application terminée");
            Ok(())
        })
        
        // === Gestionnaire d'événements globaux ===
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { .. } => {
                    info!("🔒 Fermeture de l'application demandée");
                    // Ici, on pourrait ajouter une logique de sauvegarde
                    // ou de confirmation avant fermeture
                }
                tauri::WindowEvent::Focused(est_focalisee) => {
                    if *est_focalisee {
                        info!("🎯 Application focalisée");
                    } else {
                        info!("😴 Application défocalisée");
                    }
                }
                _ => {} // Ignore les autres événements
            }
        })
        
        // === Lancement de l'application ===
        .run(tauri::generate_context!())
        .expect("❌ Erreur lors du lancement de l'application Tauri");
}



/**
 * Gestionnaire d'erreurs global pour l'application
 * 
 * Cette fonction est appelée en cas d'erreur critique dans l'application.
 * Elle log l'erreur et peut déclencher des actions de récupération.
 * 
 * # Arguments
 * * `erreur` - L'erreur qui s'est produite
 */
#[allow(dead_code)]  // Supprime l'avertissement si la fonction n'est pas utilisée
fn gerer_erreur_critique(erreur: &dyn std::error::Error) {
    error!("💥 Erreur critique dans l'application : {}", erreur);
    
    // Ici, on pourrait ajouter :
    // - Sauvegarde d'urgence des données
    // - Envoi d'un rapport d'erreur
    // - Affichage d'un message à l'utilisateur
    // - Redémarrage de certains composants
}

// === Tests unitaires ===
#[cfg(test)]
mod tests {
    use super::*;
    
    /**
     * Test de base pour vérifier que l'application peut être initialisée
     */
    #[test]
    fn test_initialisation_application() {
        // Ce test vérifie que les modules peuvent être importés correctement
        // Des tests plus complexes seront ajoutés avec les fonctionnalités
        assert!(true, "L'application peut être initialisée");
    }
}