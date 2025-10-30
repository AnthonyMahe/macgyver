/**
 * Module des commandes de productivité
 * 
 * Ce module contient toutes les commandes Tauri pour gérer :
 * - Les notifications Pomodoro
 * - La sauvegarde/chargement des notes
 * - La sauvegarde/chargement des tâches
 * 
 * Toutes les données sont sauvegardées localement dans des fichiers JSON
 * dans le répertoire de données de l'application.
 */

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use log::{info, warn, error};

// === STRUCTURES DE DONNÉES ===

/**
 * Structure représentant une note rapide
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: u64,
    pub contenu: String,
    pub date_creation: String,
}

/**
 * Structure représentant une tâche de la todolist
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tache {
    pub id: u64,
    pub texte: String,
    pub terminee: bool,
    pub date_creation: String,
}

// === COMMANDES POMODORO ===

/**
 * Commande pour notifier la fin d'un cycle Pomodoro
 * 
 * Cette commande peut être étendue pour :
 * - Afficher une notification système
 * - Jouer un son
 * - Enregistrer les statistiques
 * 
 * # Arguments
 * * `cycle` - Numéro du cycle Pomodoro terminé
 * 
 * # Retour
 * * `Result<String, String>` - Message de confirmation ou erreur
 */
#[tauri::command]
pub async fn notifier_pomodoro_termine(cycle: u32) -> Result<String, String> {
    info!("🍅 Cycle Pomodoro #{} terminé", cycle);
    
    // Ici on pourrait ajouter :
    // - Notification système native
    // - Son de notification
    // - Sauvegarde des statistiques
    
    let message = if cycle % 4 == 0 {
        format!("Excellent ! Cycle #{} terminé. Prenez une pause longue (15 min) !", cycle)
    } else {
        format!("Bravo ! Cycle #{} terminé. Prenez une pause courte (5 min) !", cycle)
    };
    
    Ok(message)
}

// === COMMANDES NOTES ===

/**
 * Sauvegarde la liste des notes dans un fichier JSON
 * 
 * # Arguments
 * * `notes` - Liste des notes à sauvegarder
 * 
 * # Retour
 * * `Result<String, String>` - Message de confirmation ou erreur
 */
#[tauri::command]
pub async fn sauvegarder_notes(notes: Vec<Note>) -> Result<String, String> {
    info!("💾 Sauvegarde de {} notes", notes.len());
    
    match obtenir_chemin_fichier_notes() {
        Ok(chemin) => {
            // Crée le répertoire parent si nécessaire
            if let Some(parent) = chemin.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    let message_erreur = format!("Impossible de créer le répertoire : {}", e);
                    error!("❌ {}", message_erreur);
                    return Err(message_erreur);
                }
            }
            
            // Sérialise et sauvegarde les notes
            match serde_json::to_string_pretty(&notes) {
                Ok(json) => {
                    match fs::write(&chemin, json) {
                        Ok(_) => {
                            info!("✅ Notes sauvegardées dans : {:?}", chemin);
                            Ok(format!("{} notes sauvegardées avec succès", notes.len()))
                        }
                        Err(e) => {
                            let message_erreur = format!("Erreur d'écriture du fichier : {}", e);
                            error!("❌ {}", message_erreur);
                            Err(message_erreur)
                        }
                    }
                }
                Err(e) => {
                    let message_erreur = format!("Erreur de sérialisation JSON : {}", e);
                    error!("❌ {}", message_erreur);
                    Err(message_erreur)
                }
            }
        }
        Err(e) => {
            error!("❌ Impossible d'obtenir le chemin du fichier notes : {}", e);
            Err(e)
        }
    }
}

/**
 * Charge la liste des notes depuis le fichier JSON
 * 
 * # Retour
 * * `Result<Vec<Note>, String>` - Liste des notes ou erreur
 */
#[tauri::command]
pub async fn charger_notes() -> Result<Vec<Note>, String> {
    info!("📖 Chargement des notes");
    
    match obtenir_chemin_fichier_notes() {
        Ok(chemin) => {
            if !chemin.exists() {
                info!("📝 Aucun fichier de notes existant, retour d'une liste vide");
                return Ok(Vec::new());
            }
            
            match fs::read_to_string(&chemin) {
                Ok(contenu) => {
                    match serde_json::from_str::<Vec<Note>>(&contenu) {
                        Ok(notes) => {
                            info!("✅ {} notes chargées depuis : {:?}", notes.len(), chemin);
                            Ok(notes)
                        }
                        Err(e) => {
                            let message_erreur = format!("Erreur de désérialisation JSON : {}", e);
                            warn!("⚠️  {}", message_erreur);
                            // Retourne une liste vide plutôt qu'une erreur pour ne pas bloquer l'app
                            Ok(Vec::new())
                        }
                    }
                }
                Err(e) => {
                    let message_erreur = format!("Erreur de lecture du fichier : {}", e);
                    warn!("⚠️  {}", message_erreur);
                    Ok(Vec::new())
                }
            }
        }
        Err(e) => {
            warn!("⚠️  Impossible d'obtenir le chemin du fichier notes : {}", e);
            Ok(Vec::new())
        }
    }
}

// === COMMANDES TÂCHES ===

/**
 * Sauvegarde la liste des tâches dans un fichier JSON
 * 
 * # Arguments
 * * `taches` - Liste des tâches à sauvegarder
 * 
 * # Retour
 * * `Result<String, String>` - Message de confirmation ou erreur
 */
#[tauri::command]
pub async fn sauvegarder_taches(taches: Vec<Tache>) -> Result<String, String> {
    info!("💾 Sauvegarde de {} tâches", taches.len());
    
    match obtenir_chemin_fichier_taches() {
        Ok(chemin) => {
            // Crée le répertoire parent si nécessaire
            if let Some(parent) = chemin.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    let message_erreur = format!("Impossible de créer le répertoire : {}", e);
                    error!("❌ {}", message_erreur);
                    return Err(message_erreur);
                }
            }
            
            // Sérialise et sauvegarde les tâches
            match serde_json::to_string_pretty(&taches) {
                Ok(json) => {
                    match fs::write(&chemin, json) {
                        Ok(_) => {
                            info!("✅ Tâches sauvegardées dans : {:?}", chemin);
                            Ok(format!("{} tâches sauvegardées avec succès", taches.len()))
                        }
                        Err(e) => {
                            let message_erreur = format!("Erreur d'écriture du fichier : {}", e);
                            error!("❌ {}", message_erreur);
                            Err(message_erreur)
                        }
                    }
                }
                Err(e) => {
                    let message_erreur = format!("Erreur de sérialisation JSON : {}", e);
                    error!("❌ {}", message_erreur);
                    Err(message_erreur)
                }
            }
        }
        Err(e) => {
            error!("❌ Impossible d'obtenir le chemin du fichier tâches : {}", e);
            Err(e)
        }
    }
}

/**
 * Charge la liste des tâches depuis le fichier JSON
 * 
 * # Retour
 * * `Result<Vec<Tache>, String>` - Liste des tâches ou erreur
 */
#[tauri::command]
pub async fn charger_taches() -> Result<Vec<Tache>, String> {
    info!("📖 Chargement des tâches");
    
    match obtenir_chemin_fichier_taches() {
        Ok(chemin) => {
            if !chemin.exists() {
                info!("📋 Aucun fichier de tâches existant, retour d'une liste vide");
                return Ok(Vec::new());
            }
            
            match fs::read_to_string(&chemin) {
                Ok(contenu) => {
                    match serde_json::from_str::<Vec<Tache>>(&contenu) {
                        Ok(taches) => {
                            info!("✅ {} tâches chargées depuis : {:?}", taches.len(), chemin);
                            Ok(taches)
                        }
                        Err(e) => {
                            let message_erreur = format!("Erreur de désérialisation JSON : {}", e);
                            warn!("⚠️  {}", message_erreur);
                            // Retourne une liste vide plutôt qu'une erreur
                            Ok(Vec::new())
                        }
                    }
                }
                Err(e) => {
                    let message_erreur = format!("Erreur de lecture du fichier : {}", e);
                    warn!("⚠️  {}", message_erreur);
                    Ok(Vec::new())
                }
            }
        }
        Err(e) => {
            warn!("⚠️  Impossible d'obtenir le chemin du fichier tâches : {}", e);
            Ok(Vec::new())
        }
    }
}

// === FONCTIONS UTILITAIRES ===

/**
 * Obtient le chemin du fichier de sauvegarde des notes
 * 
 * # Retour
 * * `Result<PathBuf, String>` - Chemin du fichier ou erreur
 */
fn obtenir_chemin_fichier_notes() -> Result<PathBuf, String> {
    obtenir_repertoire_donnees()
        .map(|mut chemin| {
            chemin.push("notes.json");
            chemin
        })
}

/**
 * Obtient le chemin du fichier de sauvegarde des tâches
 * 
 * # Retour
 * * `Result<PathBuf, String>` - Chemin du fichier ou erreur
 */
fn obtenir_chemin_fichier_taches() -> Result<PathBuf, String> {
    obtenir_repertoire_donnees()
        .map(|mut chemin| {
            chemin.push("taches.json");
            chemin
        })
}

/**
 * Obtient le répertoire de données de l'application
 * 
 * Utilise le répertoire de données utilisateur standard selon l'OS :
 * - Windows : %APPDATA%/mon-application/
 * - macOS : ~/Library/Application Support/mon-application/
 * - Linux : ~/.local/share/mon-application/
 * 
 * # Retour
 * * `Result<PathBuf, String>` - Chemin du répertoire ou erreur
 */
fn obtenir_repertoire_donnees() -> Result<PathBuf, String> {
    match dirs_next::data_dir() {
        Some(mut chemin) => {
            chemin.push("mon-application");
            Ok(chemin)
        }
        None => {
            Err("Impossible de déterminer le répertoire de données utilisateur".to_string())
        }
    }
}

// === TESTS UNITAIRES ===
#[cfg(test)]
mod tests {
    use super::*;
    
    /**
     * Test de création d'une note
     */
    #[test]
    fn test_creation_note() {
        let note = Note {
            id: 1,
            contenu: "Test note".to_string(),
            date_creation: "2024-01-01 12:00:00".to_string(),
        };
        
        assert_eq!(note.id, 1);
        assert_eq!(note.contenu, "Test note");
    }
    
    /**
     * Test de création d'une tâche
     */
    #[test]
    fn test_creation_tache() {
        let tache = Tache {
            id: 1,
            texte: "Test tâche".to_string(),
            terminee: false,
            date_creation: "2024-01-01 12:00:00".to_string(),
        };
        
        assert_eq!(tache.id, 1);
        assert_eq!(tache.texte, "Test tâche");
        assert!(!tache.terminee);
    }
    
    /**
     * Test d'obtention du répertoire de données
     */
    #[test]
    fn test_repertoire_donnees() {
        let resultat = obtenir_repertoire_donnees();
        assert!(resultat.is_ok(), "Le répertoire de données devrait être accessible");
        
        let chemin = resultat.unwrap();
        assert!(chemin.to_string_lossy().contains("mon-application"));
    }
}