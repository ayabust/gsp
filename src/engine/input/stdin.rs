//! Source d'entrée: STDIN
//!
//! Lit le texte depuis l'entrée standard.

use std::io::{self, Read};
use tracing::debug;
use crate::engine::input::trait_::InputEngine;
use crate::error::{Result, InputError};

/// Source d'entrée depuis STDIN
pub struct StdinInput;

impl StdinInput {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StdinInput {
    fn default() -> Self {
        Self::new()
    }
}

impl InputEngine for StdinInput {
    fn name(&self) -> &'static str {
        "stdin"
    }
    
    fn get_text(&self) -> Result<String> {
        debug!("Lecture depuis STDIN");
        
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| InputError::UnsupportedSource(format!("Erreur de lecture STDIN: {}", e)))?;
        
        if buffer.is_empty() {
            return Err(InputError::NoText.into());
        }
        
        debug!("Texte récupéré: {} caractères", buffer.len());
        Ok(buffer)
    }
    
    fn is_available(&self) -> bool {
        // STDIN est toujours disponible
        true
    }
}
