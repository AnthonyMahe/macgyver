/**
 * Module de conversion d'images pour MacGyver
 * 
 * Fonctionnalités :
 * - Conversion entre formats (JPG, PNG, WebP, BMP, TIFF, GIF)
 * - Redimensionnement d'images
 * - Optimisation de qualité
 * - Traitement par lots
 */

use serde::{Deserialize, Serialize};
use std::path::Path;
use image::ImageFormat;
use log::{info, error};
use crate::erreurs::{ErreurApplication, ResultatApplication};
use crate::utilitaires::progression::creer_progression_image;
use crate::utilitaires::fichiers::{verifier_fichier_existe, creer_dossier_recursif, formater_taille_fichier};

// === Types de données ===

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoImage {
    pub nom: String,
    pub chemin: String,
    pub format: String,
    pub largeur: u32,
    pub hauteur: u32,
    pub taille_octets: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionsConversion {
    pub format_sortie: String,
    pub qualite: Option<u8>,
    pub largeur_max: Option<u32>,
    pub hauteur_max: Option<u32>,
    pub conserver_ratio: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionsSuppressionFond {
    /// Couleur de fond à supprimer (format hex: "#FFFFFF")
    pub couleur_fond: String,
    /// Tolérance pour la suppression (0-255)
    pub tolerance: u8,
    /// Adoucir les bords
    pub adoucir_bords: bool,
    /// Rayon d'adoucissement
    pub rayon_adoucissement: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultatConversion {
    pub succes: bool,
    pub fichier_origine: String,
    pub fichier_sortie: String,
    pub format_origine: String,
    pub format_sortie: String,
    pub taille_avant: u64,
    pub taille_apres: u64,
    pub reduction_pourcent: f32,
    pub message: String,
}

// === Commandes Tauri ===

/**
 * Obtient les informations d'une image
 */
#[tauri::command]
pub async fn obtenir_info_image(chemin_fichier: String) -> Result<InfoImage, String> {
    info!("📷 Analyse de l'image : {}", chemin_fichier);
    
    match analyser_image(&chemin_fichier).await {
        Ok(info) => {
            info!("✅ Image analysée : {}x{} pixels", info.largeur, info.hauteur);
            Ok(info)
        }
        Err(e) => {
            error!("❌ Erreur lors de l'analyse : {}", e);
            Err(format!("Impossible d'analyser l'image : {}", e))
        }
    }
}

/**
 * Convertit une image vers un autre format
 */
#[tauri::command]
pub async fn convertir_image(
    chemin_entree: String,
    chemin_sortie: String,
    options: OptionsConversion
) -> Result<ResultatConversion, String> {
    info!("🔄 Conversion : {} -> {}", chemin_entree, chemin_sortie);
    
    match convertir_image_interne(&chemin_entree, &chemin_sortie, &options).await {
        Ok(resultat) => {
            info!("✅ Conversion réussie : -{:.1}% de taille", resultat.reduction_pourcent);
            Ok(resultat)
        }
        Err(e) => {
            error!("❌ Erreur de conversion : {}", e);
            Err(format!("Échec de la conversion : {}", e))
        }
    }
}

/**
 * Obtient les formats d'images supportés
 */
#[tauri::command]
pub async fn obtenir_formats_supportes() -> Result<Vec<String>, String> {
    Ok(vec![
        "JPEG".to_string(),
        "PNG".to_string(),
        "WebP".to_string(),
        "BMP".to_string(),
        "TIFF".to_string(),
        "GIF".to_string(),
    ])
}

/**
 * Supprime le fond d'une image et l'exporte en PNG transparent
 */
#[tauri::command]
pub async fn supprimer_fond_image(
    chemin_entree: String,
    chemin_sortie: String,
    options: OptionsSuppressionFond
) -> Result<ResultatConversion, String> {
    info!("🎨 Suppression de fond : {} -> {}", chemin_entree, chemin_sortie);
    
    match supprimer_fond_interne(&chemin_entree, &chemin_sortie, &options).await {
        Ok(resultat) => {
            info!("✅ Suppression de fond réussie");
            Ok(resultat)
        }
        Err(e) => {
            error!("❌ Erreur de suppression de fond : {}", e);
            Err(format!("Échec de la suppression de fond : {}", e))
        }
    }
}

// === Fonctions internes ===

async fn analyser_image(chemin: &str) -> ResultatApplication<InfoImage> {
    let path = Path::new(chemin);
    
    // Vérifier que le fichier existe
    if !path.exists() {
        return Err(ErreurApplication::Donnees {
            message: "Le fichier image n'existe pas".to_string(),
        });
    }
    
    // Obtenir les métadonnées du fichier
    let metadata = tokio::fs::metadata(path).await.map_err(|e| {
        ErreurApplication::Systeme {
            message: format!("Impossible de lire les métadonnées : {}", e),
        }
    })?;
    
    // Charger l'image
    let img = image::open(path).map_err(|e| {
        ErreurApplication::Donnees {
            message: format!("Impossible de charger l'image : {}", e),
        }
    })?;
    
    // Détecter le format
    let format = detecter_format_image(path)?;
    
    Ok(InfoImage {
        nom: path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        chemin: chemin.to_string(),
        format: format.to_string(),
        largeur: img.width(),
        hauteur: img.height(),
        taille_octets: metadata.len(),
    })
}

async fn convertir_image_interne(
    chemin_entree: &str,
    chemin_sortie: &str,
    options: &OptionsConversion
) -> ResultatApplication<ResultatConversion> {
    let path_entree = Path::new(chemin_entree);
    let path_sortie = Path::new(chemin_sortie);
    
    // Créer une barre de progression stylée avec les utilitaires
    let progression = creer_progression_image("Conversion d'image");
    
    // Étape 1: Vérification et lecture du fichier (20%)
    progression.mettre_a_jour(20, Some("Vérification du fichier...")).await;
    
    // Vérifier que le fichier existe avec nos utilitaires
    if !verifier_fichier_existe(chemin_entree).await {
        progression.terminer_avec_erreur("Fichier source introuvable");
        return Err(ErreurApplication::Validation {
            message: format!("Le fichier source n'existe pas : {}", chemin_entree),
        });
    }
    
    let taille_avant = tokio::fs::metadata(path_entree).await
        .map_err(|e| {
            progression.terminer_avec_erreur("Erreur de lecture des métadonnées");
            ErreurApplication::Systeme {
                message: format!("Impossible de lire le fichier d'origine : {}", e),
            }
        })?
        .len();
    
    // Étape 2: Chargement de l'image (40%)
    progression.mettre_a_jour(40, Some("Chargement de l'image...")).await;
    
    let mut img = image::open(path_entree).map_err(|e| {
        progression.terminer_avec_erreur("Impossible de charger l'image");
        ErreurApplication::Donnees {
            message: format!("Impossible de charger l'image : {}", e),
        }
    })?;
    
    // Étape 3: Redimensionnement si nécessaire (60%)
    if let (Some(largeur_max), Some(hauteur_max)) = (options.largeur_max, options.hauteur_max) {
        progression.mettre_a_jour(60, Some("Redimensionnement...")).await;
        
        if options.conserver_ratio {
            img = img.thumbnail(largeur_max, hauteur_max);
        } else {
            img = img.resize_exact(largeur_max, hauteur_max, image::imageops::FilterType::Lanczos3);
        }
    } else {
        progression.mettre_a_jour(60, Some("Pas de redimensionnement nécessaire")).await;
    }
    
    // Étape 4: Préparation de la conversion (70%)
    progression.mettre_a_jour(70, Some("Préparation de la conversion...")).await;
    
    let format_sortie = match options.format_sortie.to_lowercase().as_str() {
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "webp" => ImageFormat::WebP,
        "bmp" => ImageFormat::Bmp,
        "tiff" => ImageFormat::Tiff,
        "gif" => ImageFormat::Gif,
        _ => {
            progression.terminer_avec_erreur("Format non supporté");
            return Err(ErreurApplication::Validation {
                message: format!("Format non supporté : {}", options.format_sortie),
            });
        }
    };
    
    // Créer le dossier de sortie si nécessaire avec nos utilitaires
    if let Some(parent) = path_sortie.parent() {
        let chemin_parent = parent.to_string_lossy().to_string();
        creer_dossier_recursif(&chemin_parent).await.map_err(|e| {
            progression.terminer_avec_erreur("Impossible de créer le dossier");
            ErreurApplication::Systeme {
                message: format!("Impossible de créer le dossier : {}", e),
            }
        })?;
    }
    
    // Étape 5: Sauvegarde de l'image (90%)
    progression.mettre_a_jour(90, Some("Sauvegarde...")).await;
    
    match format_sortie {
        ImageFormat::Jpeg => {
            let qualite = options.qualite.unwrap_or(85);
            let mut output = std::fs::File::create(path_sortie).map_err(|e| {
                progression.terminer_avec_erreur("Erreur de création du fichier");
                ErreurApplication::Systeme {
                    message: format!("Impossible de créer le fichier : {}", e),
                }
            })?;
            
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, qualite);
            img.write_with_encoder(encoder).map_err(|e| {
                progression.terminer_avec_erreur("Erreur d'encodage JPEG");
                ErreurApplication::Systeme {
                    message: format!("Erreur d'encodage JPEG : {}", e),
                }
            })?;
        }
        _ => {
            img.save_with_format(path_sortie, format_sortie).map_err(|e| {
                progression.terminer_avec_erreur("Erreur de sauvegarde");
                ErreurApplication::Systeme {
                    message: format!("Erreur de sauvegarde : {}", e),
                }
            })?;
        }
    }
    
    // Étape 6: Finalisation (100%)
    progression.mettre_a_jour(100, Some("Finalisation...")).await;
    
    let taille_apres = tokio::fs::metadata(path_sortie).await
        .map_err(|e| {
            progression.terminer_avec_erreur("Erreur de vérification du fichier");
            ErreurApplication::Systeme {
                message: format!("Impossible de lire le fichier de sortie : {}", e),
            }
        })?
        .len();
    
    // Calculer la réduction
    let reduction_pourcent = if taille_avant > 0 {
        ((taille_avant as f32 - taille_apres as f32) / taille_avant as f32) * 100.0
    } else {
        0.0
    };
    
    let format_origine = detecter_format_image(path_entree)?;
    
    // Terminer la barre de progression avec succès
    let message_final = format!(
        "Conversion terminée: {} → {} ({:.1}% de réduction)", 
        formater_taille_fichier(taille_avant),
        formater_taille_fichier(taille_apres),
        reduction_pourcent
    );
    progression.terminer_avec_succes(&message_final);
    
    Ok(ResultatConversion {
        succes: true,
        fichier_origine: chemin_entree.to_string(),
        fichier_sortie: chemin_sortie.to_string(),
        format_origine: format_origine.to_string(),
        format_sortie: options.format_sortie.clone(),
        taille_avant,
        taille_apres,
        reduction_pourcent,
        message: format!(
            "Conversion réussie : {} -> {} ({:.1}% de réduction)",
            format_origine,
            options.format_sortie,
            reduction_pourcent
        ),
    })
}

async fn supprimer_fond_interne(
    chemin_entree: &str,
    chemin_sortie: &str,
    options: &OptionsSuppressionFond
) -> ResultatApplication<ResultatConversion> {
    let path_entree = Path::new(chemin_entree);
    let path_sortie = Path::new(chemin_sortie);
    
    // Créer une barre de progression pour la suppression de fond
    let progression = creer_progression_image("Suppression de fond");
    
    // Étape 1: Vérification du fichier (20%)
    progression.mettre_a_jour(20, Some("Vérification du fichier...")).await;
    
    if !verifier_fichier_existe(chemin_entree).await {
        progression.terminer_avec_erreur("Fichier source introuvable");
        return Err(ErreurApplication::Validation {
            message: format!("Le fichier source n'existe pas : {}", chemin_entree),
        });
    }
    
    let taille_avant = tokio::fs::metadata(path_entree).await
        .map_err(|e| {
            progression.terminer_avec_erreur("Erreur de lecture des métadonnées");
            ErreurApplication::Systeme {
                message: format!("Impossible de lire le fichier d'origine : {}", e),
            }
        })?
        .len();
    
    // Étape 2: Chargement de l'image (40%)
    progression.mettre_a_jour(40, Some("Chargement de l'image...")).await;
    
    let img = image::open(path_entree).map_err(|e| {
        progression.terminer_avec_erreur("Impossible de charger l'image");
        ErreurApplication::Donnees {
            message: format!("Impossible de charger l'image : {}", e),
        }
    })?;
    
    // Convertir en RGBA pour gérer la transparence
    let mut img_rgba = img.to_rgba8();
    
    // Étape 3: Parsing de la couleur de fond (50%)
    progression.mettre_a_jour(50, Some("Analyse de la couleur de fond...")).await;
    
    let couleur_cible = parser_couleur_hex(&options.couleur_fond)?;
    
    // Étape 4: Suppression du fond (80%)
    progression.mettre_a_jour(80, Some("Suppression du fond...")).await;
    
    supprimer_couleur_fond(&mut img_rgba, couleur_cible, options.tolerance);
    
    // Étape 5: Adoucissement des bords si demandé (90%)
    if options.adoucir_bords {
        progression.mettre_a_jour(90, Some("Adoucissement des bords...")).await;
        adoucir_bords_transparence(&mut img_rgba, options.rayon_adoucissement);
    } else {
        progression.mettre_a_jour(90, Some("Finalisation...")).await;
    }
    
    // Étape 6: Sauvegarde en PNG (100%)
    progression.mettre_a_jour(100, Some("Sauvegarde en PNG...")).await;
    
    // Créer le dossier de sortie si nécessaire
    if let Some(parent) = path_sortie.parent() {
        let chemin_parent = parent.to_string_lossy().to_string();
        creer_dossier_recursif(&chemin_parent).await.map_err(|e| {
            progression.terminer_avec_erreur("Impossible de créer le dossier");
            ErreurApplication::Systeme {
                message: format!("Impossible de créer le dossier : {}", e),
            }
        })?;
    }
    
    // Sauvegarder en PNG (seul format supportant la transparence)
    img_rgba.save_with_format(path_sortie, image::ImageFormat::Png).map_err(|e| {
        progression.terminer_avec_erreur("Erreur de sauvegarde PNG");
        ErreurApplication::Systeme {
            message: format!("Erreur de sauvegarde PNG : {}", e),
        }
    })?;
    
    let taille_apres = tokio::fs::metadata(path_sortie).await
        .map_err(|e| {
            progression.terminer_avec_erreur("Erreur de vérification du fichier");
            ErreurApplication::Systeme {
                message: format!("Impossible de lire le fichier de sortie : {}", e),
            }
        })?
        .len();
    
    let reduction_pourcent = if taille_avant > 0 {
        ((taille_avant as f32 - taille_apres as f32) / taille_avant as f32) * 100.0
    } else {
        0.0
    };
    
    let format_origine = detecter_format_image(path_entree)?;
    
    let message_final = format!(
        "Fond supprimé: {} → PNG transparent", 
        formater_taille_fichier(taille_avant)
    );
    progression.terminer_avec_succes(&message_final);
    
    Ok(ResultatConversion {
        succes: true,
        fichier_origine: chemin_entree.to_string(),
        fichier_sortie: chemin_sortie.to_string(),
        format_origine: format_origine.to_string(),
        format_sortie: "PNG".to_string(),
        taille_avant,
        taille_apres,
        reduction_pourcent,
        message: format!(
            "Suppression de fond réussie : {} -> PNG transparent",
            format_origine
        ),
    })
}

fn parser_couleur_hex(hex: &str) -> ResultatApplication<[u8; 3]> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return Err(ErreurApplication::Validation {
            message: "La couleur doit être au format #RRGGBB".to_string(),
        });
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| {
        ErreurApplication::Validation {
            message: "Couleur hexadécimale invalide".to_string(),
        }
    })?;
    
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| {
        ErreurApplication::Validation {
            message: "Couleur hexadécimale invalide".to_string(),
        }
    })?;
    
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| {
        ErreurApplication::Validation {
            message: "Couleur hexadécimale invalide".to_string(),
        }
    })?;
    
    Ok([r, g, b])
}

fn supprimer_couleur_fond(img: &mut image::RgbaImage, couleur_cible: [u8; 3], tolerance: u8) {
    for pixel in img.pixels_mut() {
        let [r, g, b, a] = pixel.0;
        
        // Calculer la distance euclidienne entre la couleur du pixel et la couleur cible
        let distance = (
            ((r as i32 - couleur_cible[0] as i32).pow(2) +
             (g as i32 - couleur_cible[1] as i32).pow(2) +
             (b as i32 - couleur_cible[2] as i32).pow(2)) as f32
        ).sqrt();
        
        // Si la couleur est proche de la couleur cible (dans la tolérance)
        if distance <= tolerance as f32 {
            // Rendre le pixel transparent
            pixel.0[3] = 0;
        } else if distance <= (tolerance as f32 * 1.5) {
            // Transition douce pour les pixels proches
            let alpha_factor = (distance - tolerance as f32) / (tolerance as f32 * 0.5);
            pixel.0[3] = (a as f32 * alpha_factor.min(1.0)) as u8;
        }
    }
}

fn adoucir_bords_transparence(img: &mut image::RgbaImage, rayon: u8) {
    if rayon == 0 {
        return;
    }
    
    let (width, height) = img.dimensions();
    let mut nouvelle_img = img.clone();
    
    for y in 0..height {
        for x in 0..width {
            let pixel_actuel = img.get_pixel(x, y);
            
            // Si le pixel est partiellement transparent, adoucir
            if pixel_actuel.0[3] > 0 && pixel_actuel.0[3] < 255 {
                let mut alpha_total = 0u32;
                let mut count = 0u32;
                
                // Examiner les pixels voisins
                for dy in -(rayon as i32)..=(rayon as i32) {
                    for dx in -(rayon as i32)..=(rayon as i32) {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        
                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            let voisin = img.get_pixel(nx as u32, ny as u32);
                            alpha_total += voisin.0[3] as u32;
                            count += 1;
                        }
                    }
                }
                
                if count > 0 {
                    let alpha_moyen = (alpha_total / count) as u8;
                    nouvelle_img.get_pixel_mut(x, y).0[3] = alpha_moyen;
                }
            }
        }
    }
    
    *img = nouvelle_img;
}

fn detecter_format_image(path: &Path) -> ResultatApplication<&'static str> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => Ok("JPEG"),
            "png" => Ok("PNG"),
            "webp" => Ok("WebP"),
            "bmp" => Ok("BMP"),
            "tiff" | "tif" => Ok("TIFF"),
            "gif" => Ok("GIF"),
            _ => Err(ErreurApplication::Validation {
                message: format!("Extension de fichier non reconnue : {}", ext),
            }),
        },
        None => Err(ErreurApplication::Validation {
            message: "Impossible de déterminer le format du fichier".to_string(),
        }),
    }
}

// === Tests ===
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detecter_format_image() {
        assert_eq!(detecter_format_image(Path::new("test.jpg")).unwrap(), "JPEG");
        assert_eq!(detecter_format_image(Path::new("test.png")).unwrap(), "PNG");
        assert_eq!(detecter_format_image(Path::new("test.webp")).unwrap(), "WebP");
    }
}