//! Traduction via LibreTranslate
//!
//! Utilise l'API LibreTranslate (open source) pour la traduction.

use reqwest::blocking::Client;
use serde_json::{json, Value};
use tracing::{debug, error};
use crate::engine::translate::trait_::TranslateEngine;
use crate::error::{Result, TranslationError};

/// Moteur de traduction LibreTranslate
pub struct LibreTranslate {
    api_url: String,
    client: Client,
}

impl LibreTranslate {
    pub fn new() -> Self {
        Self {
            api_url: "https://libretranslate.com/translate".to_string(),
            client: Client::new(),
        }
    }
    
    pub fn with_url(api_url: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            client: Client::new(),
        }
    }
}

impl Default for LibreTranslate {
    fn default() -> Self {
        Self::new()
    }
}

impl TranslateEngine for LibreTranslate {
    fn name(&self) -> &'static str {
        "libretranslate"
    }
    
    fn translate(&self, text: &str, source_lang: &str, target_lang: &str) -> Result<String> {
        debug!("Traduction LibreTranslate: {} -> {}", source_lang, target_lang);
        
        let body = json!({
            "q": text,
            "source": if source_lang == "auto" { "auto" } else { source_lang.split('-').next().unwrap_or(source_lang) },
            "target": target_lang.split('-').next().unwrap_or(target_lang),
            "format": "text"
        });
        
        let response = self.client.post(&self.api_url)
            .json(&body)
            .send()
            .map_err(|e| TranslationError::NetworkError(format!("Erreur réseau: {}", e)))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            error!("LibreTranslate API error: {} - {}", status, error_text);
            return Err(TranslationError::TranslationFailed(
                format!("API error: {} - {}", status, error_text)
            ).into());
        }
        
        let json: Value = response.json()
            .map_err(|e| TranslationError::InvalidApiResponse(format!("Parsing JSON échoué: {}", e)))?;
        
        let translated = json["translatedText"]
            .as_str()
            .ok_or_else(|| TranslationError::InvalidApiResponse("Champ 'translatedText' manquant".to_string()))?;
        
        debug!("Traduction terminée: {} caractères", translated.len());
        Ok(translated.to_string())
    }
    
    fn is_available(&self) -> bool {
        // Test rapide de connectivité
        self.client.get("https://libretranslate.com")
            .send()
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}
