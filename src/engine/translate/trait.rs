//! Trait pour les moteurs de traduction
//!
//! Définit l'interface commune pour tous les moteurs de traduction.

use crate::error::Result;

/// Interface commune pour tous les moteurs de traduction
pub trait TranslateEngine {
    /// Nom du moteur
    fn name(&self) -> &'static str;
    
    /// Traduit un texte
    /// 
    /// # Arguments
    /// * `text` - Le texte à traduire
    /// * `source_lang` - Langue source (ou "auto" pour détection automatique)
    /// * `target_lang` - Langue cible
    fn translate(&self, text: &str, source_lang: &str, target_lang: &str) -> Result<String>;
    
    /// Vérifie si le moteur est disponible
    fn is_available(&self) -> bool;
}
