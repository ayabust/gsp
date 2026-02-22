//! Trait pour les moteurs Text-to-Speech
//!
//! Définit l'interface commune que tous les moteurs TTS doivent implémenter.

use crate::error::{Result, TtsError};
use std::path::PathBuf;

/// Interface commune pour tous les moteurs TTS
pub trait TtsEngine {
    /// Nom du moteur
    fn name(&self) -> &'static str;
    
    /// Génère un fichier audio depuis du texte
    /// 
    /// # Arguments
    /// * `text` - Le texte à synthétiser
    /// * `output_path` - Chemin où sauvegarder le fichier WAV
    /// * `lang` - Code de langue (ex: "fr-FR")
    /// * `speed` - Vitesse de lecture (1.0 = normale)
    fn generate(&self, text: &str, output_path: &PathBuf, lang: &str, speed: f32) -> Result<()>;
    
    /// Vérifie si le moteur est disponible sur le système
    fn is_available(&self) -> bool;
    
    /// Liste des langues supportées
    fn supported_languages(&self) -> Vec<&'static str>;
}
