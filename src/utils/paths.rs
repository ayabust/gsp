//! Gestion des chemins et répertoires
//!
//! Ce module centralise la gestion des chemins de fichiers
//! et s'assure que les répertoires nécessaires existent.

use crate::error::{GspError, Result};
use std::path::{Path, PathBuf};

/// Gestionnaire de chemins pour GSP
pub struct PathManager {
    temp_dir: PathBuf,
    cache_dir: PathBuf,
    log_dir: Option<PathBuf>,
}

impl PathManager {
    /// Crée un nouveau PathManager avec les répertoires par défaut
    pub fn new() -> Self {
        Self {
            temp_dir: PathBuf::from("/tmp/gsp"),
            cache_dir: PathBuf::from("/tmp/gsp/audio"),
            log_dir: None,
        }
    }

    /// Crée un PathManager depuis une configuration
    pub fn from_config(temp_dir: PathBuf, cache_dir: PathBuf, log_dir: Option<PathBuf>) -> Self {
        Self {
            temp_dir,
            cache_dir,
            log_dir,
        }
    }

    /// S'assure que tous les répertoires existent
    pub fn ensure_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(&self.temp_dir).map_err(|e| {
            GspError::ConfigError(format!("Impossible de créer le répertoire temp: {}", e))
        })?;

        std::fs::create_dir_all(&self.cache_dir).map_err(|e| {
            GspError::ConfigError(format!("Impossible de créer le répertoire cache: {}", e))
        })?;

        if let Some(ref log_dir) = self.log_dir {
            std::fs::create_dir_all(log_dir).map_err(|e| {
                GspError::ConfigError(format!("Impossible de créer le répertoire de logs: {}", e))
            })?;
        }

        Ok(())
    }

    /// Chemin pour un fichier audio temporaire
    pub fn temp_audio_path(&self, name: &str) -> PathBuf {
        self.temp_dir.join(format!("{}.wav", name))
    }

    /// Chemin pour un fichier audio en cache
    pub fn cache_audio_path(&self, hash: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.wav", hash))
    }

    /// Chemin pour un fichier de log
    pub fn log_path(&self, name: &str) -> Option<PathBuf> {
        self.log_dir.as_ref().map(|dir| dir.join(name))
    }

    /// Nettoie les fichiers temporaires
    pub fn cleanup_temp(&self) -> Result<()> {
        if self.temp_dir.exists() {
            std::fs::remove_dir_all(&self.temp_dir)
                .map_err(|e| GspError::ConfigError(format!("Erreur lors du nettoyage: {}", e)))?;
            std::fs::create_dir_all(&self.temp_dir).map_err(|e| {
                GspError::ConfigError(format!("Impossible de recréer le répertoire temp: {}", e))
            })?;
        }
        Ok(())
    }

    /// Getter pour le répertoire temp
    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Getter pour le répertoire cache
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
}

impl Default for PathManager {
    fn default() -> Self {
        Self::new()
    }
}
