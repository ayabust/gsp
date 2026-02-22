//! Trait pour les moteurs de lecture audio
//!
//! Définit l'interface commune pour la lecture des fichiers audio.

use crate::error::{Result, AudioError};
use std::path::Path;

/// Interface commune pour tous les moteurs de lecture audio
pub trait AudioPlayer {
    /// Nom du lecteur
    fn name(&self) -> &'static str;
    
    /// Joue un fichier audio
    fn play(&self, path: &Path) -> Result<()>;
    
    /// Arrête la lecture en cours
    fn stop(&self) -> Result<()>;
    
    /// Définit le volume (0.0 à 1.0)
    fn set_volume(&mut self, volume: f32);
}
