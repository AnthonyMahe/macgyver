// Module des utilitaires de fichiers
// 
// Toutes les fonctions ont été supprimées car elles n'étaient pas utilisées.
// Ce fichier est conservé pour les futures fonctions de gestion de fichiers.

use std::path::Path;
use tokio::fs;
use log::{info, warn, debug};
use crate::erreurs::{ErreurUtilitaire, ResultatUtilitaire};

// === Fonctions utilisées ===

/**
 * Vérifie si un fichier existe
 * 
 * # Arguments
 * * `chemin` - Le chemin du fichier à vérifier
 * 
 * # Retour
 * * `bool` - `true` si le fichier existe, `false` sinon
 */
pub async fn verifier_fichier_existe(chemin: &str) -> bool {
    debug!("🔍 Vérification de l'existence du fichier : {}", chemin);
    
    let existe = Path::new(chemin).exists();
    
    if existe {
        debug!("✅ Fichier trouvé : {}", chemin);
    } else {
        debug!("❌ Fichier non trouvé : {}", chemin);
    }
    
    existe
}

/**
 * Crée un dossier de manière récursive
 * 
 * # Arguments
 * * `chemin` - Le chemin du dossier à créer
 * 
 * # Retour
 * * `ResultatUtilitaire<()>` - Succès ou erreur
 */
pub async fn creer_dossier_recursif(chemin: &str) -> ResultatUtilitaire<()> {
    debug!("📁 Création du dossier : {}", chemin);
    
    fs::create_dir_all(chemin).await.map_err(|e| {
        warn!("❌ Erreur lors de la création du dossier {} : {}", chemin, e);
        ErreurUtilitaire::Validation {
            message: format!("Impossible de créer le dossier {} : {}", chemin, e),
        }
    })?;
    
    info!("✅ Dossier créé avec succès : {}", chemin);
    Ok(())
}

/**
 * Formate la taille d'un fichier en format lisible
 * 
 * # Arguments
 * * `taille_bytes` - La taille en bytes
 * 
 * # Retour
 * * `String` - Taille formatée (ex: "1.5 MB")
 */
pub fn formater_taille_fichier(taille_bytes: u64) -> String {
    const UNITES: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if taille_bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut taille = taille_bytes as f64;
    let mut unite_index = 0;
    
    while taille >= 1024.0 && unite_index < UNITES.len() - 1 {
        taille /= 1024.0;
        unite_index += 1;
    }
    
    if unite_index == 0 {
        format!("{} {}", taille_bytes, UNITES[unite_index])
    } else {
        format!("{:.1} {}", taille, UNITES[unite_index])
    }
}