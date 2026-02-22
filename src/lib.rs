//! GSP - Bibliothèque de lecteur d'écran
//!
//! Cette crate fournit les composants pour la synthèse vocale,
//! la reconnaissance OCR et la traduction de texte.

pub mod engine;
pub mod error;
pub mod config;
pub mod pipeline;
pub mod utils;

// Ré-exports pour une utilisation facile
pub use engine::{TtsEngine, TtsFactory, InputEngine, TranslateEngine, AudioPlayer};
pub use error::{GspError, Result};
pub use config::Config;
pub use pipeline::ProcessingPipeline;
pub use utils::Language;
