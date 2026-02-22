//! OCR avec Tesseract
//!
//! Utilise la crate leptess (bindings Rust pour Tesseract) pour la reconnaissance de texte.

use std::path::Path;
use tracing::{debug, error};
use crate::error::{Result, InputError};

/// Moteur OCR basé sur Tesseract
pub struct TesseractOcr {
    lang: String,
}

impl TesseractOcr {
    pub fn new() -> Self {
        Self {
            lang: "fra".to_string(),
        }
    }
    
    pub fn with_lang(lang: &str) -> Self {
        // Convertir les codes de langue BCP 47 en codes Tesseract
        let tess_lang = match lang {
            "fr-FR" | "fr" => "fra",
            "en-US" | "en-GB" | "en" => "eng",
            "de-DE" | "de" => "deu",
            "es-ES" | "es" => "spa",
            "it-IT" | "it" => "ita",
            _ => "fra",
        };
        
        Self {
            lang: tess_lang.to_string(),
        }
    }
    
    /// Reconnaît le texte depuis une image
    pub fn recognize(&self, image_path: &Path) -> Result<String> {
        debug!("Reconnaissance OCR de: {:?}", image_path);
        
        if !image_path.exists() {
            return Err(InputError::OcrFailure("Fichier image introuvable".to_string()).into());
        }
        
        // Initialiser Tesseract
        let tess = leptess::TessApi::new(Some("/usr/share/tessdata"), &self.lang)
            .map_err(|e| InputError::OcrFailure(format!("Initialisation Tesseract échouée: {}", e)))?;
        
        // Charger l'image
        tess.set_image(image_path)
            .map_err(|e| InputError::OcrFailure(format!("Chargement de l'image échoué: {}", e)))?;
        
        // Reconnaître le texte
        let text = tess.get_utf8_text()
            .map_err(|e| InputError::OcrFailure(format!("Reconnaissance échouée: {}", e)))?;
        
        debug!("Texte OCR récupéré: {} caractères", text.len());
        Ok(text)
    }
    
    /// Vérifie si Tesseract est disponible
    pub fn is_available() -> bool {
        // Vérifier si le répertoire de données Tesseract existe
        Path::new("/usr/share/tessdata").exists()
    }
}

impl Default for TesseractOcr {
    fn default() -> Self {
        Self::new()
    }
}
