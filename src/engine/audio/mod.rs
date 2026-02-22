//! Module de lecture audio
//!
//! Fournit les lecteurs audio et leurs interfaces.

pub mod trait_;
pub mod rodio;

pub use trait_::AudioPlayer;
pub use rodio::RodioPlayer;
