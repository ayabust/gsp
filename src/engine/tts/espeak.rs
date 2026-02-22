//! Moteur TTS eSpeak
//!
//! Utilise espeak pour la synthèse vocale.

use std::path::PathBuf;
use std::process::Command;
use tracing::{debug, error};
use crate::engine::tts::trait::TtsEngine;
use crate::error::{Result, TtsError};

/// Moteur TTS eSpeak
pub struct Espeak {
    default_lang: String,
}

impl Espeak {
    pub fn new() -> Self {
        Self {
            default_lang: "fr".to_string(),
        }
    }
    
    pub fn with_lang(lang: &str) -> Self {
        Self {
            default_lang: lang.to_string(),
        }
    }
}

impl Default for Espeak {
    fn default() -> Self {
        Self::new()
    }
}

impl TtsEngine for Espeak {
    fn name(&self) -> &'static str {
        "espeak"
    }
    
    fn generate(&self, text: &str, output_path: &PathBuf, lang: &str, speed: f32) -> Result<()> {
        debug!("Génération audio avec eSpeak: {} caractères", text.len());
        
        // Conversion speed: 1.0 = 175 wpm par défaut pour espeak
        let wpm = (175.0 * speed) as u32;
        
        let output = Command::new("espeak")
            .args([
                "-w", output_path.to_str().ok_or_else(|| {
                    TtsError::GenerationFailed("Chemin invalide".to_string())
                })?,
                "-v", lang,
                "-s", &wpm.to_string(),
                text,
            ])
            .output()
            .map_err(|e| TtsError::EngineExecutionError(format!("espeak échec: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("espeak a échoué: {}", stderr);
            return Err(TtsError::GenerationFailed(stderr.to_string()));
        }
        
        if !output_path.exists() {
            return Err(TtsError::GenerationFailed("Aucun fichier audio généré".to_string()));
        }
        
        debug!("Fichier audio généré: {:?}", output_path);
        Ok(())
    }
    
    fn is_available(&self) -> bool {
        Command::new("espeak")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    fn supported_languages(&self) -> Vec<&'static str> {
        vec!["fr", "en", "en-us", "en-gb", "de", "es", "it", "pt", "nl", "pl", "ru"]
    }
}
