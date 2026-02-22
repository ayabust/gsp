//! Traduction via Argos Translate
//!
//! Utilise Argos Translate (offline, basé sur OpenNMT) pour la traduction.
//! Note: Nécessite l'installation des modèles de langue.

use std::process::Command;
use tracing::{debug, error};
use crate::engine::translate::trait_::TranslateEngine;
use crate::error::{Result, TranslationError};

/// Moteur de traduction Argos Translate (CLI)
pub struct ArgosTranslate {
    default_source: String,
}

impl ArgosTranslate {
    pub fn new() -> Self {
        Self {
            default_source: "auto".to_string(),
        }
    }
}

impl Default for ArgosTranslate {
    fn default() -> Self {
        Self::new()
    }
}

impl TranslateEngine for ArgosTranslate {
    fn name(&self) -> &'static str {
        "argos"
    }
    
    fn translate(&self, text: &str, source_lang: &str, target_lang: &str) -> Result<String> {
        debug!("Traduction Argos: {} -> {}", source_lang, target_lang);
        
        // Argos Translate CLI: argostranslate.translate --from-code --to-code
        let from = if source_lang == "auto" { "en" } else { source_lang.split('-').next().unwrap_or("en") };
        let to = target_lang.split('-').next().unwrap_or("fr");
        
        let output = Command::new("argostranslate")
            .args(["translate", "--from-code", from, "--to-code", to])
            .arg(text)
            .output()
            .map_err(|e| TranslationError::EngineUnavailable(
                format!("argostranslate non installé: {}", e)
            ))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Argos Translate échec: {}", stderr);
            return Err(TranslationError::TranslationFailed(stderr.to_string()).into());
        }
        
        let translated = String::from_utf8_lossy(&output.stdout).to_string();
        debug!("Traduction terminée: {} caractères", translated.len());
        Ok(translated)
    }
    
    fn is_available(&self) -> bool {
        Command::new("argostranslate")
            .arg("--help")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}
