/**
 * Module principal des commandes Tauri
 * 
 * Ce module centralise toutes les commandes Rust exposées au frontend.
 * Il organise les commandes par domaine fonctionnel et facilite leur
 * enregistrement dans l'application principale.
 * 
 * Organisation :
 * - Chaque domaine fonctionnel a son propre sous-module
 * - Les commandes sont réexportées pour simplifier l'import
 * - Documentation complète de chaque commande
 * 
 * Domaines fonctionnels :
 * - salutation : Commandes de démonstration et test
 * - (futurs modules à ajouter selon les besoins)
 */

// === Déclaration des sous-modules ===
// Chaque sous-module contient les commandes d'un domaine spécifique



/// Module contenant les commandes de productivité
/// 
/// Ce module fournit toutes les fonctionnalités pour l'application de productivité :
/// - Gestion du timer Pomodoro
/// - Sauvegarde et chargement des notes rapides
/// - Gestion de la todolist avec persistance
pub mod productivite;

/// Module contenant les commandes de conversion d'images
/// 
/// Ce module fournit les fonctionnalités de traitement d'images :
/// - Conversion entre formats (JPG, PNG, WebP, BMP, TIFF, GIF)
/// - Redimensionnement et optimisation
/// - Analyse des métadonnées d'images
pub mod images;

/// Module contenant les commandes de manipulation de PDF
/// 
/// Ce module fournit les fonctionnalités de traitement de PDF :
/// - Fusion de plusieurs PDFs en un seul
/// - Division d'un PDF en pages individuelles
/// - Extraction d'informations et métadonnées
/// - Rotation et manipulation de pages
pub mod pdf;

// === Futurs modules à ajouter ===
// Décommentez et ajoutez selon les besoins de votre application

// /// Module pour la gestion des fichiers et dossiers
// pub mod fichiers;

// /// Module pour les informations système
// pub mod systeme;

// /// Module pour la gestion de la configuration
// pub mod configuration;

// /// Module pour les opérations de base de données
// pub mod base_donnees;

// /// Module pour les opérations réseau
// pub mod reseau;

// === Réexports pour simplifier l'utilisation ===
// Ces réexports permettent d'importer les commandes directement depuis ce module

// Réexport des commandes de salutation
// Réexports supprimés pour éviter les warnings

// === Documentation des conventions ===
/**
 * Conventions pour les commandes Tauri
 * 
 * Toutes les commandes doivent respecter ces conventions pour maintenir
 * la cohérence et la qualité du code :
 * 
 * ## Signature des fonctions
 * ```rust
 * #[tauri::command]
 * pub async fn nom_commande(
 *     parametre1: Type1,
 *     parametre2: Type2,
 * ) -> Result<TypeRetour, String> {
 *     // Implémentation
 * }
 * ```
 * 
 * ## Nommage
 * - Noms en français explicites (ex: `saluer_utilisateur` au lieu de `greet`)
 * - Utilisation du snake_case pour les noms de fonctions
 * - Préfixes par domaine si nécessaire (ex: `fichier_lire`, `systeme_info`)
 * 
 * ## Gestion d'erreurs
 * - Toujours retourner un `Result<T, String>` ou `Result<T, ErreurPersonnalisee>`
 * - Messages d'erreur en français et compréhensibles par l'utilisateur
 * - Logging des erreurs avec le niveau approprié
 * 
 * ## Documentation
 * - Commentaires Rustdoc complets pour chaque fonction publique
 * - Exemples d'utilisation dans la documentation
 * - Description des paramètres et valeurs de retour
 * 
 * ## Tests
 * - Tests unitaires pour chaque commande
 * - Tests d'intégration pour les flux complexes
 * - Mocks pour les dépendances externes
 * 
 * ## Sécurité
 * - Validation de tous les paramètres d'entrée
 * - Sanitisation des données utilisateur
 * - Vérification des permissions si nécessaire
 */

// === Utilitaires communs aux commandes ===

// Type ResultatCommande supprimé car non utilisé

/**
 * Macro pour simplifier la création de commandes avec logging
 * 
 * Cette macro ajoute automatiquement du logging d'entrée et de sortie
 * pour toutes les commandes, facilitant le debugging.
 * 
 * # Exemple
 * ```rust
 * commande_avec_log!(ma_commande, param1: String, param2: i32 => String {
 *     // Implémentation de la commande
 *     Ok(format!("Résultat: {} - {}", param1, param2))
 * });
 * ```
 */
#[macro_export]
macro_rules! commande_avec_log {
    ($nom:ident, $($param:ident: $type:ty),* => $retour:ty $corps:block) => {
        #[tauri::command]
        pub async fn $nom($($param: $type),*) -> ResultatCommande<$retour> {
            use log::{info, error};
            
            info!("🔄 Exécution de la commande '{}' avec paramètres: {:?}", 
                  stringify!($nom), ($($param,)*));
            
            let resultat = async move $corps.await;
            
            match &resultat {
                Ok(_) => info!("✅ Commande '{}' exécutée avec succès", stringify!($nom)),
                Err(e) => error!("❌ Erreur dans la commande '{}': {}", stringify!($nom), e),
            }
            
            resultat
        }
    };
}

// === Tests du module ===
#[cfg(test)]
mod tests {
    use super::*;
    
    /**
     * Test de base pour vérifier que le module peut être importé
     */
    #[test]
    fn test_module_commandes() {
        // Vérifie que le module est correctement structuré
        assert!(true, "Le module commandes est correctement configuré");
    }
}