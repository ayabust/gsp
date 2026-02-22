//! Trait pour les sources d'entrée de texte
//!
//! Définit l'interface commune pour toutes les sources d'entrée.

use crate::error::Result;

/// Interface commune pour toutes les sources d'entrée
pub trait InputEngine {
    /// Nom de la source
    fn name(&self) -> &'static str;
    
    /// Récupère le texte depuis la source
    fn get_text(&self) -> Result<String>;
    
    /// Vérifie si la source est disponible
    fn is_available(&self) -> bool;
}
