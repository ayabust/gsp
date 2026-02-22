//! Gestion d'erreurs centralisée pour GSP
//!
//! Ce module définit tous les types d'erreurs possibles dans l'application
//! en utilisant le pattern `thiserror` pour une gestion ergonomique.

use thiserror::Error;

/// Type de résultat principal pour GSP
pub type Result<T> = std::result::Result<T, GspError>;

/// Erreurs principales de l'application GSP
#[derive(Error, Debug)]
pub enum GspError {
    #[error("Erreur d'entrée: {0}")]
    InputError(#[from] InputError),

    #[error("Erreur TTS: {0}")]
    TtsError(#[from] TtsError),

    #[error("Erreur de traduction: {0}")]
    TranslationError(#[from] TranslationError),

    #[error("Erreur audio: {0}")]
    AudioError(#[from] AudioError),

    #[error("Erreur de configuration: {0}")]
    ConfigError(String),

    #[error("Une autre instance est déjà en cours d'exécution")]
    InstanceAlreadyRunning,

    #[error("Aucun texte à lire")]
    NoText,

    #[error("Erreur interne: {0}")]
    InternalError(String),
}

/// Erreurs liées aux sources d'entrée (clipboard, sélection, fichier, OCR)
#[derive(Error, Debug)]
pub enum InputError {
    #[error("Impossible de lire la sélection: {0}")]
    SelectionError(String),

    #[error("Presse-papier vide")]
    EmptyClipboard,

    #[error("Fichier introuvable: {0}")]
    FileNotFound(String),

    #[error("Échec de la lecture du fichier: {0}")]
    FileReadError(String),

    #[error("Échec OCR: {0}")]
    OcrFailure(String),

    #[error("Source d'entrée non supportée: {0}")]
    UnsupportedSource(String),

    #[error("Erreur d'initialisation du clipboard: {0}")]
    ClipboardInitError(String),
}

/// Erreurs liées à la synthèse vocale (TTS)
#[derive(Error, Debug)]
pub enum TtsError {
    #[error("Moteur TTS non disponible: {0}")]
    EngineUnavailable(String),

    #[error("Échec de génération audio: {0}")]
    GenerationFailed(String),

    #[error("Langue non supportée: {0}")]
    UnsupportedLanguage(String),

    #[error("Fichier audio introuvable: {0}")]
    AudioFileNotFound(String),

    #[error("Erreur d'exécution du moteur TTS: {0}")]
    EngineExecutionError(String),

    #[error("Paramètres TTS invalides: {0}")]
    InvalidParameters(String),
}

/// Erreurs liées à la traduction
#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("Moteur de traduction non disponible: {0}")]
    EngineUnavailable(String),

    #[error("Échec de la traduction: {0}")]
    TranslationFailed(String),

    #[error("Langue source non supportée: {0}")]
    UnsupportedSourceLanguage(String),

    #[error("Langue cible non supportée: {0}")]
    UnsupportedTargetLanguage(String),

    #[error("Erreur réseau lors de la traduction: {0}")]
    NetworkError(String),

    #[error("Réponse invalide du service de traduction: {0}")]
    InvalidResponse(String),

    #[error("Limite de traduction atteinte")]
    RateLimitExceeded,
}

/// Erreurs liées à la lecture audio
#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Impossible de lire le fichier audio: {0}")]
    PlaybackError(String),

    #[error("Périphérique audio non disponible: {0}")]
    DeviceUnavailable(String),

    #[error("Format audio non supporté: {0}")]
    UnsupportedFormat(String),

    #[error("Erreur d'initialisation audio: {0}")]
    InitError(String),

    #[error("Erreur de stream audio: {0}")]
    StreamError(String),
}

// ============================================================================
// Implémentations pour la conversion d'erreurs externes
// ============================================================================

impl From<std::io::Error> for GspError {
    fn from(err: std::io::Error) -> Self {
        GspError::InternalError(err.to_string())
    }
}

impl From<std::io::Error> for InputError {
    fn from(err: std::io::Error) -> Self {
        InputError::FileReadError(err.to_string())
    }
}

impl From<std::io::Error> for AudioError {
    fn from(err: std::io::Error) -> Self {
        AudioError::PlaybackError(err.to_string())
    }
}

impl From<reqwest::Error> for TranslationError {
    fn from(err: reqwest::Error) -> Self {
        TranslationError::NetworkError(err.to_string())
    }
}

// ============================================================================
// Macros utilitaires pour la gestion d'erreurs
// ============================================================================

/// Macro pour retourner une erreur GSP avec un message personnalisé
#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(format!($fmt, $($arg)*).into())
    };
}

/// Macro pour assurer qu'une condition est vraie, sinon retourne une erreur
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !($cond) {
            bail!($err);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            bail!($fmt, $($arg)*);
        }
    };
}
