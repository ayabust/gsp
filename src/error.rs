//! Types d'erreurs centralisés pour GSP
//!
//! Ce module définit tous les types d'erreurs utilisés dans l'application
//! en utilisant `thiserror` pour une gestion ergonomique.

use thiserror::Error;

/// Erreur principale de l'application GSP
#[derive(Error, Debug)]
pub enum GspError {
    #[error("Erreur de lecture: {0}")]
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

    #[error("Erreur de pipeline: {0}")]
    PipelineError(String),
}

/// Erreurs liées aux sources d'entrée
#[derive(Error, Debug)]
pub enum InputError {
    #[error("Impossible de lire la sélection: {0}")]
    SelectionError(String),

    #[error("Presse-papier vide")]
    EmptyClipboard,

    #[error("Fichier introuvable: {0}")]
    FileNotFound(String),

    #[error("Échec OCR: {0}")]
    OcrFailure(String),

    #[error("Erreur de presse-papier: {0}")]
    ClipboardError(String),

    #[error("Source d'entrée non supportée: {0}")]
    UnsupportedSource(String),
}

/// Erreurs liées au Text-to-Speech
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
}

/// Erreurs liées à la traduction
#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("Moteur de traduction non disponible: {0}")]
    EngineUnavailable(String),

    #[error("Échec de la traduction: {0}")]
    TranslationFailed(String),

    #[error("Erreur réseau: {0}")]
    NetworkError(String),

    #[error("Langue source non supportée: {0}")]
    UnsupportedSourceLanguage(String),

    #[error("Langue cible non supportée: {0}")]
    UnsupportedTargetLanguage(String),

    #[error("Réponse API invalide: {0}")]
    InvalidApiResponse(String),
}

/// Erreurs liées à la lecture audio
#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Périphérique audio non disponible: {0}")]
    DeviceUnavailable(String),

    #[error("Échec de la lecture: {0}")]
    PlaybackFailed(String),

    #[error("Format audio non supporté: {0}")]
    UnsupportedFormat(String),

    #[error("Erreur de décodage: {0}")]
    DecodeError(String),

    #[error("Erreur d'arrêt: {0}")]
    StopError(String),
}

/// Type Result alias pour GSP
pub type Result<T> = std::result::Result<T, GspError>;
