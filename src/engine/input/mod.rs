//! Module des sources d'entrée
//!
//! Fournit les différentes sources d'entrée de texte.

pub mod trait_;
pub mod clipboard;
pub mod selection;
pub mod stdin;
pub mod file;
pub mod ocr;

pub use trait_::InputEngine;
pub use clipboard::ClipboardInput;
pub use selection::SelectionInput;
pub use stdin::StdinInput;
pub use file::FileInput;
pub use ocr::tesseract::TesseractOcr;
