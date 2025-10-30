/**
 * Module principal des utilitaires Rust
 * 
 * Ce module centralise toutes les fonctions utilitaires réutilisables
 * de l'application. Il organise les utilitaires par domaine fonctionnel
 * et respecte strictement le principe DRY (Don't Repeat Yourself).
 * 
 * Organisation :
 * - Chaque domaine fonctionnel a son propre sous-module
 * - Les fonctions sont réexportées pour simplifier l'import
 * - Documentation complète de chaque utilitaire
 * - Tests unitaires pour chaque fonction
 * 
 * Domaines fonctionnels :
 * - formatage : Fonctions de formatage de chaînes et données
 * - validation : Fonctions de validation et vérification
 * - (futurs modules à ajouter selon les besoins)
 */

// === Déclaration des sous-modules ===
// Chaque sous-module contient les utilitaires d'un domaine spécifique

/// Module contenant les fonctions de formatage de données
/// 
/// Ce module fournit des fonctions pour formater, nettoyer et transformer
/// les données de manière cohérente dans toute l'application.
pub mod formatage;

/// Module contenant les fonctions de validation
/// 
/// Ce module fournit des fonctions pour valider les données d'entrée,
/// vérifier leur intégrité et s'assurer de leur sécurité.
pub mod validation;

/// Module contenant les utilitaires de progression
/// 
/// Ce module fournit des outils pour créer des barres de progression
/// stylées et cohérentes dans toute l'application.
pub mod progression;

/// Module contenant les utilitaires de gestion de fichiers
/// 
/// Ce module fournit des fonctions réutilisables pour les opérations
/// courantes sur les fichiers et dossiers.
pub mod fichiers;

// === Futurs modules à ajouter ===
// Décommentez et ajoutez selon les besoins de votre application

// /// Module pour les opérations sur les fichiers
// pub mod fichiers;

// /// Module pour les opérations cryptographiques
// pub mod cryptographie;

// /// Module pour les opérations de date et heure
// pub mod dates;

// /// Module pour les opérations mathématiques
// pub mod mathematiques;

// /// Module pour les opérations de configuration
// pub mod configuration;

// === Réexports pour simplifier l'utilisation ===
// Ces réexports permettent d'importer les utilitaires directement depuis ce module

// Modules disponibles (réexports supprimés pour éviter les warnings)

// Import des types d'erreurs depuis le module dédié
// Import ErreurUtilitaire supprimé car non utilisé

// Constantes globales supprimées car non utilisées

// === Macros utilitaires ===

/**
 * Macro pour créer des fonctions de validation avec logging
 * 
 * Cette macro simplifie la création de fonctions de validation
 * en ajoutant automatiquement du logging et une gestion d'erreurs cohérente.
 * 
 * # Exemple
 * ```rust
 * validation_avec_log!(valider_age, age: i32 => bool {
 *     age >= 0 && age <= 150
 * }, "L'âge doit être entre 0 et 150 ans");
 * ```
 */
#[macro_export]
macro_rules! validation_avec_log {
    ($nom:ident, $param:ident: $type:ty => $retour:ty $condition:block, $message_erreur:expr) => {
        /// Fonction de validation générée automatiquement
        pub fn $nom($param: $type) -> ResultatUtilitaire<$retour> {
            use log::{debug, warn};
            
            debug!("🔍 Validation '{}' avec paramètre: {:?}", stringify!($nom), $param);
            
            let resultat = $condition;
            
            if resultat {
                debug!("✅ Validation '{}' réussie", stringify!($nom));
                Ok(resultat)
            } else {
                warn!("❌ Validation '{}' échouée: {}", stringify!($nom), $message_erreur);
                Err(ErreurUtilitaire::Validation {
                    message: $message_erreur.to_string()
                })
            }
        }
    };
}

/**
 * Macro pour créer des fonctions de formatage avec logging
 * 
 * Cette macro simplifie la création de fonctions de formatage
 * en ajoutant automatiquement du logging et une gestion d'erreurs cohérente.
 * 
 * # Exemple
 * ```rust
 * formatage_avec_log!(formater_nom, nom: String => String {
 *     nom.trim().to_lowercase()
 * });
 * ```
 */
#[macro_export]
macro_rules! formatage_avec_log {
    ($nom:ident, $param:ident: $type:ty => $retour:ty $transformation:block) => {
        /// Fonction de formatage générée automatiquement
        pub fn $nom($param: $type) -> ResultatUtilitaire<$retour> {
            use log::debug;
            
            debug!("🔧 Formatage '{}' avec paramètre: {:?}", stringify!($nom), $param);
            
            let resultat = $transformation;
            
            debug!("✅ Formatage '{}' terminé", stringify!($nom));
            Ok(resultat)
        }
    };
}

// === Fonctions utilitaires génériques ===

/**
 * Convertit une erreur utilitaire en chaîne de caractères pour l'interface utilisateur
 * 
 * Cette fonction standardise la conversion des erreurs en messages
 * compréhensibles par l'utilisateur final.
 * 
 * # Arguments
 * * `erreur` - L'erreur à convertir
 * 
 * # Retour
 * * `String` - Message d'erreur formaté pour l'utilisateur
 */
// Fonctions erreur_vers_message et generer_identifiant_unique supprimées car non utilisées

// === Tests du module ===
#[cfg(test)]
mod tests {
    use super::*;
    
    /**
     * Test de génération d'identifiant unique
     */
    #[test]
    fn test_generer_identifiant_unique() {
        let id1 = generer_identifiant_unique();
        let id2 = generer_identifiant_unique();
        
        // Vérifie que les IDs sont différents
        assert_ne!(id1, id2, "Les identifiants doivent être uniques");
        
        // Vérifie le format de base
        assert!(id1.contains("_"), "L'identifiant doit contenir des underscores");
        assert!(id1.len() > 10, "L'identifiant doit avoir une longueur suffisante");
    }
    
    /**
     * Test de conversion d'erreur en message
     */
    #[test]
    fn test_erreur_vers_message() {
        let erreur = ErreurUtilitaire::Validation {
            message: "Test de validation".to_string()
        };
        
        let message = erreur_vers_message(&erreur);
        assert!(message.contains("Données invalides"), "Le message doit contenir le préfixe approprié");
        assert!(message.contains("Test de validation"), "Le message doit contenir le message original");
    }
}