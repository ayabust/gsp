//! Pipeline de traitement GSP
//!
//! Orchestre le flux de traitement: Input → Translate → TTS → Audio

use crate::engine::{InputEngine, TtsEngine, TranslateEngine, AudioPlayer};
use crate::error::{Result, GspError};
use std::path::PathBuf;
use tracing::{info, debug};

/// Pipeline de traitement complet
pub struct ProcessingPipeline<I, T, Tr, A> {
    input: I,
    translator: Option<Tr>,
    tts: T,
    audio: A,
    text_processor: TextProcessor,
    lang: String,
    speed: f32,
}

impl<I, T, Tr, A> ProcessingPipeline<I, T, Tr, A>
where
    I: InputEngine,
    T: TtsEngine,
    Tr: TranslateEngine,
    A: AudioPlayer,
{
    /// Crée un nouveau pipeline sans traduction
    pub fn new(input: I, tts: T, audio: A) -> Self {
        Self {
            input,
            translator: None,
            tts,
            audio,
            text_processor: TextProcessor::default(),
            lang: "fr-FR".to_string(),
            speed: 1.0,
        }
    }
    
    /// Ajoute un traducteur au pipeline
    pub fn with_translation(mut self, translator: Tr) -> Self {
        self.translator = Some(translator);
        self
    }
    
    /// Définit la langue cible
    pub fn with_lang(mut self, lang: &str) -> Self {
        self.lang = lang.to_string();
        self
    }
    
    /// Définit la vitesse de lecture
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    
    /// Exécute le pipeline complet
    pub fn execute(&mut self) -> Result<()> {
        info!("Démarrage du pipeline GSP");
        
        // Étape 1: Récupération du texte
        let raw_text = self.input.get_text()?;
        debug!("Texte brut récupéré: {} caractères", raw_text.len());
        
        // Étape 2: Prétraitement du texte
        let processed_text = self.text_processor.process(raw_text)?;
        info!("Texte traité: {} caractères", processed_text.len());
        
        // Étape 3: Traduction (optionnelle)
        let final_text = if let Some(ref translator) = self.translator {
            let translated = translator.translate(&processed_text, "auto", &self.lang)?;
            info!("Texte traduit");
            translated
        } else {
            processed_text
        };
        
        // Étape 4: Synthèse vocale
        let output_path = std::env::temp_dir().join(format!("gsp_{}.wav", std::process::id()));
        self.tts.generate(&final_text, &output_path, &self.lang, self.speed)?;
        info!("Synthèse vocale générée: {:?}", output_path);
        
        // Étape 5: Lecture audio
        self.audio.play(&output_path)?;
        info!("Lecture terminée");
        
        // Nettoyage
        let _ = std::fs::remove_file(&output_path);
        
        Ok(())
    }
}

/// Processeur de texte pour le prétraitement
#[derive(Default)]
pub struct TextProcessor {
    lowercase: bool,
    remove_special: bool,
}

impl TextProcessor {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_lowercase(mut self, lowercase: bool) -> Self {
        self.lowercase = lowercase;
        self
    }
    
    pub fn with_remove_special(mut self, remove: bool) -> Self {
        self.remove_special = remove;
        self
    }
    
    /// Traite le texte selon les options configurées
    pub fn process(&self, text: String) -> Result<String> {
        let mut result = text;
        
        if self.lowercase {
            result = result.to_lowercase();
        }
        
        if self.remove_special {
            result = remove_special_characters(&result);
        }
        
        result = trim_whitespace(&result);
        
        if result.is_empty() {
            return Err(GspError::NoText.into());
        }
        
        Ok(result)
    }
}

/// Supprime les caractères spéciaux indésirables
fn remove_special_characters(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == ',' || *c == '.' || *c == '!' || *c == '?')
        .collect()
}

/// Trim les espaces multiples
fn trim_whitespace(text: &str) -> String {
    let mut prev_space = false;
    let mut result = String::new();
    
    for c in text.chars() {
        if c.is_whitespace() {
            if !prev_space {
                result.push(' ');
                prev_space = true;
            }
        } else {
            result.push(c);
            prev_space = false;
        }
    }
    
    result.trim().to_string()
}
