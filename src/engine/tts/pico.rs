//! Moteur TTS Pico
//!
//! Utilise libttspico-utils (pico2wave) pour la synthèse vocale.

use std::path::PathBuf;
use std::process::Command;
use tracing::{debug, error};
use crate::engine::tts::trait::TtsEngine;
use crate::error::{Result, TtsError};

/// Moteur TTS Pico (pico2wave)
pub struct Pico {
    default_lang: String,
}

impl Pico {
    pub fn new() -> Self {
        Self {
            default_lang: "fr-FR".to_string(),
        }
    }
    
    pub fn with_lang(lang: &str) -> Self {
        Self {
            default_lang: lang.to_string(),
        }
    }
}

impl Default for Pico {
    fn default() -> Self {
        Self::new()
    }
}

impl TtsEngine for Pico {
    fn name(&self) -> &'static str {
        "pico"
    }
    
    fn generate(&self, text: &str, output_path: &PathBuf, lang: &str, speed: f32) -> Result<()> {
        debug!("Génération audio avec Pico: {} caractères", text.len());
        
        // pico2wave ne supporte pas directement la vitesse, on ignore ce paramètre
        let _ = speed;
        
        let output = Command::new("pico2wave")
            .args([
                "--wave", output_path.to_str().ok_or_else(|| {
                    TtsError::GenerationFailed("Chemin invalide".to_string())
                })?,
                "--lang", lang,
            ])
            .arg(text)
            .output()
            .map_err(|e| TtsError::EngineExecutionError(format!("pico2wave échec: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("pico2wave a échoué: {}", stderr);
            return Err(TtsError::GenerationFailed(stderr.to_string()));
        }
        
        // Vérifier que le fichier a été créé
        if !output_path.exists() {
            return Err(TtsError::GenerationFailed("Aucun fichier audio généré".to_string()));
        }
        
        debug!("Fichier audio généré: {:?}", output_path);
        Ok(())
    }
    
    fn is_available(&self) -> bool {
        Command::new("pico2wave")
            .arg("--help")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    fn supported_languages(&self) -> Vec<&'static str> {
        vec!["fr-FR", "en-US", "en-GB", "de-DE", "es-ES", "it-IT"]
    }
}
