//! Moteurs de traitement pour GSP
//!
//! Ce module regroupe tous les moteurs (TTS, Input, Translate, Audio)
//! utilisés par l'application.

pub mod tts;
pub mod input;
pub mod translate;
pub mod audio;

pub use tts::{TtsEngine, TtsFactory, Pico, Espeak, EspeakNg};
pub use input::{InputEngine, ClipboardInput, SelectionInput, StdinInput, FileInput, TesseractOcr};
pub use translate::{TranslateEngine, LibreTranslate, ArgosTranslate};
pub use audio::{AudioPlayer, RodioPlayer};
