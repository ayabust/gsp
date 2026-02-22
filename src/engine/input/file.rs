//! Source d'entrée: Fichier
//!
//! Lit le texte depuis un fichier.

use std::fs;
use std::path::Path;
use tracing::debug;
use crate::engine::input::trait_::InputEngine;
use crate::error::{Result, InputError};

/// Source d'entrée depuis un fichier
pub struct FileInput {
    path: String,
}

impl FileInput {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl InputEngine for FileInput {
    fn name(&self) -> &'static str {
        "file"
    }
    
    fn get_text(&self) -> Result<String> {
        let path = Path::new(&self.path);
        debug!("Lecture du fichier: {:?}", path);
        
        if !path.exists() {
            return Err(InputError::FileNotFound(self.path.clone()).into());
        }
        
        let text = fs::read_to_string(path)
            .map_err(|e| InputError::FileNotFound(format!("Impossible de lire le fichier: {}", e)))?;
        
        if text.is_empty() {
            return Err(InputError::FileNotFound("Fichier vide".to_string()).into());
        }
        
        debug!("Texte récupéré: {} caractères", text.len());
        Ok(text)
    }
    
    fn is_available(&self) -> bool {
        Path::new(&self.path).exists()
    }
}
