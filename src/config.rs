//! Gestion de configuration externalisée
//!
//! Ce module permet de charger et gérer la configuration de GSP
//! depuis un fichier TOML, avec des valeurs par défaut.

use serde::Deserialize;
use std::path::PathBuf;
use crate::error::{GspError, Result};

/// Configuration principale de GSP
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub tts: TtsConfig,
    #[serde(default)]
    pub translation: TranslationConfig,
    #[serde(default)]
    pub audio: AudioConfig,
    #[serde(default)]
    pub paths: PathsConfig,
}

/// Configuration du Text-to-Speech
#[derive(Debug, Deserialize, Clone)]
pub struct TtsConfig {
    #[serde(default = "default_engine")]
    pub default_engine: String,
    #[serde(default = "default_speed")]
    pub default_speed: f32,
    #[serde(default = "default_language")]
    pub default_language: String,
}

/// Configuration de la traduction
#[derive(Debug, Deserialize, Clone)]
pub struct TranslationConfig {
    #[serde(default = "default_translation_engine")]
    pub default_engine: String,
    #[serde(default = "default_true")]
    pub auto_detect: bool,
}

/// Configuration audio
#[derive(Debug, Deserialize, Clone)]
pub struct AudioConfig {
    pub output_device: Option<String>,
    #[serde(default = "default_volume")]
    pub volume: f32,
    #[serde(default = "default_cache_dir")]
    pub cache_dir: PathBuf,
}

/// Configuration des chemins
#[derive(Debug, Deserialize, Clone)]
pub struct PathsConfig {
    #[serde(default = "default_temp_dir")]
    pub temp_dir: PathBuf,
    pub log_dir: Option<PathBuf>,
}

impl Config {
    /// Charge la configuration depuis le fichier de config ou utilise les valeurs par défaut
    pub fn load() -> Result<Self> {
        let config_path = Self::default_path();
        
        if config_path.exists() {
            // Charger depuis le fichier
            let content = std::fs::read_to_string(&config_path)
                .map_err(|e| GspError::ConfigError(format!("Impossible de lire le fichier de config: {}", e)))?;
            
            toml::from_str(&content)
                .map_err(|e| GspError::ConfigError(format!("Erreur de parsing TOML: {}", e)))
        } else {
            // Retourner la configuration par défaut
            Ok(Self::default())
        }
    }
    
    /// Chemin par défaut du fichier de configuration
    pub fn default_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("gsp")
            .join("config.toml")
    }
    
    /// Configuration par défaut
    pub fn default() -> Self {
        Self {
            tts: TtsConfig::default(),
            translation: TranslationConfig::default(),
            audio: AudioConfig::default(),
            paths: PathsConfig::default(),
        }
    }
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            default_engine: default_engine(),
            default_speed: default_speed(),
            default_language: default_language(),
        }
    }
}

impl Default for TranslationConfig {
    fn default() -> Self {
        Self {
            default_engine: default_translation_engine(),
            auto_detect: default_true(),
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            output_device: None,
            volume: default_volume(),
            cache_dir: default_cache_dir(),
        }
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            temp_dir: default_temp_dir(),
            log_dir: None,
        }
    }
}

// Fonctions de valeurs par défaut pour serde
fn default_engine() -> String { "pico".to_string() }
fn default_speed() -> f32 { 1.0 }
fn default_language() -> String { "fr-FR".to_string() }
fn default_translation_engine() -> String { "libretranslate".to_string() }
fn default_true() -> bool { true }
fn default_volume() -> f32 { 1.0 }
fn default_cache_dir() -> PathBuf { PathBuf::from("/tmp/gsp/audio") }
fn default_temp_dir() -> PathBuf { PathBuf::from("/tmp/gsp") }
