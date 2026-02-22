//! Source d'entrée: Sélection X11
//!
//! Lit le texte depuis la sélection primaire X11 (sélection avec la souris).

use arboard::Clipboard;
use tracing::debug;
use crate::engine::input::trait_::InputEngine;
use crate::error::{Result, InputError};

/// Source d'entrée depuis la sélection X11
pub struct SelectionInput;

impl SelectionInput {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SelectionInput {
    fn default() -> Self {
        Self::new()
    }
}

impl InputEngine for SelectionInput {
    fn name(&self) -> &'static str {
        "selection"
    }
    
    fn get_text(&self) -> Result<String> {
        debug!("Lecture de la sélection X11");
        
        let clipboard = Clipboard::new()
            .map_err(|e| InputError::SelectionError(format!("Impossible d'accéder au presse-papier: {}", e)))?;
        
        // arboard lit la sélection primaire par défaut sur Linux
        let text = clipboard.get_text()
            .map_err(|e| InputError::SelectionError(format!("Erreur de lecture: {}", e)))?;
        
        if text.is_empty() {
            return Err(InputError::SelectionError("Aucune sélection disponible".to_string()).into());
        }
        
        debug!("Texte récupéré: {} caractères", text.len());
        Ok(text)
    }
    
    fn is_available(&self) -> bool {
        // Vérifie si X11 est disponible
        Clipboard::new().is_ok()
    }
}
