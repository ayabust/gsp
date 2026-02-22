//! Factory pour créer des instances de moteurs TTS
//!
//! Permet d'instancier le moteur TTS approprié selon la configuration.

use crate::engine::tts::{TtsEngine, Pico, Espeak, EspeakNg};
use crate::error::{Result, TtsError};
use tracing::debug;

/// Factory pour créer des moteurs TTS
pub struct TtsFactory;

impl TtsFactory {
    /// Crée un moteur TTS selon le nom spécifié
    pub fn create(engine_name: &str, lang: &str) -> Result<Box<dyn TtsEngine>> {
        debug!("Création du moteur TTS: {}", engine_name);
        
        let engine: Box<dyn TtsEngine> = match engine_name.to_lowercase().as_str() {
            "pico" => Box::new(Pico::with_lang(lang)),
            "espeak" => Box::new(Espeak::with_lang(lang)),
            "espeak-ng" | "espeakng" => Box::new(EspeakNg::with_lang(lang)),
            _ => {
                return Err(TtsError::EngineUnavailable(
                    format!("Moteur TTS '{}' non supporté", engine_name)
                ).into());
            }
        };
        
        // Vérifier la disponibilité
        if !engine.is_available() {
            return Err(TtsError::EngineUnavailable(
                format!("Le moteur '{}' n'est pas installé ou disponible", engine_name)
            ).into());
        }
        
        Ok(engine)
    }
    
    /// Crée le moteur TTS par défaut
    pub fn create_default(lang: &str) -> Result<Box<dyn TtsEngine>> {
        // Essayer pico en premier (meilleure qualité), puis espeak-ng, puis espeak
        let engines = ["pico", "espeak-ng", "espeak"];
        
        for engine_name in engines {
            if let Ok(engine) = Self::create(engine_name, lang) {
                debug!("Moteur TTS par défaut sélectionné: {}", engine_name);
                return Ok(engine);
            }
        }
        
        Err(TtsError::EngineUnavailable(
            "Aucun moteur TTS disponible. Installez pico2wave, espeak-ng ou espeak.".to_string()
        ).into())
    }
    
    /// Liste tous les moteurs disponibles sur le système
    pub fn list_available() -> Vec<&'static str> {
        let mut available = Vec::new();
        
        if Pico::new().is_available() {
            available.push("pico");
        }
        if EspeakNg::new().is_available() {
            available.push("espeak-ng");
        }
        if Espeak::new().is_available() {
            available.push("espeak");
        }
        
        available
    }
}
