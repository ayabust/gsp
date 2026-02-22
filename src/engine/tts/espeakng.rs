//! Moteur TTS eSpeak-NG
//!
//! Utilise espeak-ng pour la synthèse vocale (version améliorée d'eSpeak).

use std::path::PathBuf;
use std::process::Command;
use tracing::{debug, error};
use crate::engine::tts::trait::TtsEngine;
use crate::error::{Result, TtsError};

/// Moteur TTS eSpeak-NG
pub struct EspeakNg {
    default_lang: String,
}

impl EspeakNg {
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

impl Default for EspeakNg {
    fn default() -> Self {
        Self::new()
    }
}

impl TtsEngine for EspeakNg {
    fn name(&self) -> &'static str {
        "espeak-ng"
    }
    
    fn generate(&self, text: &str, output_path: &PathBuf, lang: &str, speed: f32) -> Result<()> {
        debug!("Génération audio avec eSpeak-NG: {} caractères", text.len());
        
        // Conversion speed: 1.0 = 175 wpm par défaut pour espeak-ng
        let wpm = (175.0 * speed) as u32;
        
        let output = Command::new("espeak-ng")
            .args([
                "-w", output_path.to_str().ok_or_else(|| {
                    TtsError::GenerationFailed("Chemin invalide".to_string())
                })?,
                "-v", lang,
                "-s", &wpm.to_string(),
                text,
            ])
            .output()
            .map_err(|e| TtsError::EngineExecutionError(format!("espeak-ng échec: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("espeak-ng a échoué: {}", stderr);
            return Err(TtsError::GenerationFailed(stderr.to_string()));
        }
        
        if !output_path.exists() {
            return Err(TtsError::GenerationFailed("Aucun fichier audio généré".to_string()));
        }
        
        debug!("Fichier audio généré: {:?}", output_path);
        Ok(())
    }
    
    fn is_available(&self) -> bool {
        Command::new("espeak-ng")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    fn supported_languages(&self) -> Vec<&'static str> {
        vec!["fr", "en", "en-us", "en-gb", "de", "es", "it", "pt", "nl", "pl", "ru", "ja", "zh"]
    }
}
