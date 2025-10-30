// Module de gestion de la progression
// 
// Code simplifié pour garder seulement les fonctions utilisées.

use log::{info, debug};

// === Types de progression ===

/**
 * Types de progression disponibles
 */
#[derive(Debug, Clone)]
pub enum TypeProgression {
    /// Barre pour conversion d'images
    ConversionImage,
}

/**
 * Configuration d'une barre de progression
 */
#[derive(Debug, Clone)]
pub struct ConfigProgression {
    pub type_progression: TypeProgression,
    pub message_initial: String,
}

/**
 * Gestionnaire de progression pour les opérations longues
 */
#[derive(Debug)]
pub struct GestionnaireProgression {
    config: ConfigProgression,
}

impl GestionnaireProgression {
    /**
     * Crée un nouveau gestionnaire de progression
     */
    pub fn nouveau(config: ConfigProgression) -> Self {
        debug!("📊 Création d'un gestionnaire de progression : {:?}", config.type_progression);
        
        Self {
            config,
        }
    }
    
    /**
     * Met à jour la progression
     */
    pub async fn mettre_a_jour(&self, pourcentage: u64, message: Option<&str>) {
        let message_affiche = message.unwrap_or(&self.config.message_initial);
        
        debug!("📈 Progression : {}% - {}", pourcentage, message_affiche);
        
        if pourcentage >= 100 {
            info!("✅ Progression terminée : {}", message_affiche);
        }
    }
    
    /**
     * Termine la progression avec succès
     */
    pub fn terminer(&self, message: &str) {
        info!("🎉 Progression terminée avec succès : {}", message);
    }
    
    /**
     * Termine la progression avec succès (alias)
     */
    pub fn terminer_avec_succes(&self, message: &str) {
        self.terminer(message);
    }
    
    /**
     * Termine la progression avec erreur
     */
    pub fn terminer_avec_erreur(&self, message: &str) {
        info!("❌ Progression terminée avec erreur : {}", message);
    }
}

// === Fonctions utilitaires ===

/**
 * Crée une barre de progression pour conversion d'image
 */
pub fn creer_progression_image(message: &str) -> GestionnaireProgression {
    let config = ConfigProgression {
        type_progression: TypeProgression::ConversionImage,
        message_initial: message.to_string(),
    };
    
    GestionnaireProgression::nouveau(config)
}