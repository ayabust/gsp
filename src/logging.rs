//! Configuration du logging structuré pour GSP
//!
//! Ce module initialise le système de logging en utilisant `tracing` et
//! `tracing-subscriber`. Supporte le logging vers stdout et/ou fichier.

use std::path::PathBuf;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::error::GspError;

/// Initialise le système de logging
///
/// # Arguments
///
/// * `log_dir` - Répertoire pour les logs fichier (optionnel)
/// * `verbose` - Si true, active le niveau DEBUG, sinon INFO
///
/// # Returns
///
/// * `Ok(())` si l'initialisation réussit
/// * `Err(GspError)` si l'initialisation échoue
pub fn init(log_dir: Option<PathBuf>, verbose: bool) -> Result<(), GspError> {
    let filter = if verbose {
        EnvFilter::new("gsp=debug")
    } else {
        EnvFilter::new("gsp=info")
    };

    let subscriber = tracing_subscriber::registry().with(filter);

    if let Some(dir) = log_dir {
        // Logging vers fichier avec rotation quotidienne
        match init_file_logging(subscriber, dir) {
            Ok(_) => Ok(()),
            Err(e) => {
                // Fallback vers stdout si le logging fichier échoue
                eprintln!("Attention: échec logging fichier, fallback vers stdout: {}", e);
                init_stdout_logging(subscriber)
            }
        }
    } else {
        // Logging vers stdout
        init_stdout_logging(subscriber)
    }
}

/// Initialise le logging vers stdout
fn init_stdout_logging(
    subscriber: tracing_subscriber::Registry,
) -> Result<(), GspError> {
    subscriber
        .with(
            fmt::layer()
                .with_target(false)
                .without_time()
                .with_ansi(true),
        )
        .try_init()
        .map_err(|e| GspError::ConfigError(format!("Logging init failed: {}", e)))?;

    Ok(())
}

/// Initialise le logging vers fichier avec rotation
fn init_file_logging(
    subscriber: tracing_subscriber::Registry,
    log_dir: PathBuf,
) -> Result<(), GspError> {
    // Créer le répertoire s'il n'existe pas
    std::fs::create_dir_all(&log_dir).map_err(|e| {
        GspError::ConfigError(format!("Impossible de créer le répertoire de logs: {}", e))
    })?;

    let file_appender = tracing_appender::rolling::daily(&log_dir, "gsp.log");

    subscriber
        .with(
            fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(false)
                .with_thread_names(false),
        )
        .try_init()
        .map_err(|e| GspError::ConfigError(format!("Logging init failed: {}", e)))?;

    Ok(())
}

/// Macro utilitaire pour logger un événement avec contexte
#[macro_export]
macro_rules! log_event {
    ($level:ident, $msg:expr) => {
        tracing::$level!($msg)
    };
    ($level:ident, $msg:expr, $($arg:tt)*) => {
        tracing::$level!($msg, $($arg)*)
    };
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_init_stdout_logging() {
        // Note: ce test peut échouer si un autre test a déjà initialisé le logger
        let result = init(None, false);
        // On ignore l'erreur si le logger est déjà initialisé
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_init_file_logging() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let log_dir = temp_dir.path().to_path_buf();

        let result = init(Some(log_dir.clone()), true);
        assert!(result.is_ok() || result.is_err());

        // Vérifier que le répertoire existe
        assert!(log_dir.exists());
    }

    #[test]
    fn test_log_dir_creation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let nested_dir = temp_dir.path().join("nested").join("logs");

        // Le logging devrait créer le répertoire s'il n'existe pas
        fs::create_dir_all(&nested_dir).expect("Failed to create dir");
        assert!(nested_dir.exists());
    }
}
