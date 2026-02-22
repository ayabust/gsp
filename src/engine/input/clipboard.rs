//! Source d'entrée: Presse-papier
//!
//! Lit le texte depuis le presse-papier système.

use arboard::Clipboard;
use tracing::debug;
use crate::engine::input::trait_::InputEngine;
use crate::error::{Result, InputError};

/// Source d'entrée depuis le presse-papier
pub struct ClipboardInput;

impl ClipboardInput {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClipboardInput {
    fn default() -> Self {
        Self::new()
    }
}

impl InputEngine for ClipboardInput {
    fn name(&self) -> &'static str {
        "clipboard"
    }
    
    fn get_text(&self) -> Result<String> {
        debug!("Lecture du presse-papier");
        
        let clipboard = Clipboard::new()
            .map_err(|e| InputError::ClipboardError(format!("Impossible d'accéder au presse-papier: {}", e)))?;
        
        let text = clipboard.get_text()
            .map_err(|e| InputError::ClipboardError(format!("Erreur de lecture: {}", e)))?;
        
        if text.is_empty() {
            return Err(InputError::EmptyClipboard.into());
        }
        
        debug!("Texte récupéré: {} caractères", text.len());
        Ok(text)
    }
    
    fn is_available(&self) -> bool {
        Clipboard::new().is_ok()
    }
}
