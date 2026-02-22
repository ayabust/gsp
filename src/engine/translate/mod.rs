//! Module de traduction
//!
//! Fournit les moteurs de traduction et leurs interfaces.

pub mod trait_;
pub mod libretranslate;
pub mod argos;

pub use trait_::TranslateEngine;
pub use libretranslate::LibreTranslate;
pub use argos::ArgosTranslate;
