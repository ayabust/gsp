//! Configuration du logging structuré
//!
//! Utilise `tracing` et `tracing-subscriber` pour un logging performant
//! et configurable.

use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use std::path::PathBuf;
use crate::error::{GspError, Result};

/// Initialise le système de logging
///
/// # Arguments
/// * `log_dir` - Répertoire pour les logs fichier (optionnel)
/// * `verbose` - Si true, active le niveau debug
pub fn init(log_dir: Option<PathBuf>, verbose: bool) -> Result<()> {
    let filter = if verbose {
        EnvFilter::new("gsp=debug")
    } else {
        EnvFilter::new("gsp=info")
    };
    
    let subscriber = tracing_subscriber::registry()
        .with(filter);
    
    if let Some(dir) = log_dir {
        // S'assurer que le répertoire existe
        std::fs::create_dir_all(&dir)
            .map_err(|e| GspError::ConfigError(format!("Impossible de créer le répertoire de logs: {}", e)))?;
        
        // Logging vers fichier avec rotation quotidienne
        let file_appender = tracing_appender::rolling::daily(&dir, "gsp.log");
        subscriber
            .with(fmt::layer().with_writer(file_appender))
            .try_init()
    } else {
        // Logging vers stdout
        subscriber
            .with(fmt::layer().with_target(false).without_time())
            .try_init()
    }
    .map_err(|e| GspError::ConfigError(format!("Logging init failed: {}", e)))?;
    
    Ok(())
}
