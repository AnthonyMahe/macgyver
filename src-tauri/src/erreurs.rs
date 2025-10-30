/**
 * Module de gestion d'erreurs simplifié
 * 
 * Ce module contient uniquement les types d'erreurs
 * réellement utilisés dans l'application.
 */

// Imports supprimés car non utilisés dans cette version simplifiée

// === Types d'erreurs essentiels ===

/**
 * Énumération des erreurs de validation
 */
#[derive(Debug, thiserror::Error)]
pub enum ErreurUtilitaire {
    #[error("Erreur de validation : {message}")]
    Validation { message: String },
}

/**
 * Type de résultat pour les utilitaires
 */
pub type ResultatUtilitaire<T> = Result<T, ErreurUtilitaire>;

/**
 * Énumération des erreurs d'application principales
 */
#[derive(Debug, thiserror::Error)]
pub enum ErreurApplication {
    #[error("Erreur de validation : {message}")]
    Validation { message: String },
    
    #[error("Erreur de données : {message}")]
    Donnees { message: String },
    
    #[error("Erreur système : {message}")]
    Systeme { message: String },
}

/**
 * Type de résultat pour l'application
 */
pub type ResultatApplication<T> = Result<T, ErreurApplication>;

// Implémentation de la conversion automatique
impl From<ErreurUtilitaire> for ErreurApplication {
    fn from(erreur: ErreurUtilitaire) -> Self {
        match erreur {
            ErreurUtilitaire::Validation { message } => ErreurApplication::Validation { message },
        }
    }
}