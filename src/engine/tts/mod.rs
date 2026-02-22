//! Module Text-to-Speech
//!
//! Fournit les moteurs TTS et une factory pour les instancier.

pub mod trait_;
pub mod pico;
pub mod espeak;
pub mod espeakng;
pub mod factory;

pub use trait_::TtsEngine;
pub use pico::Pico;
pub use espeak::Espeak;
pub use espeakng::EspeakNg;
pub use factory::TtsFactory;
