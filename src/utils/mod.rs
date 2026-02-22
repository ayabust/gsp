//! Utilitaires divers pour GSP

pub mod lang;
pub mod paths;
pub mod process;

pub use lang::Language;
pub use paths::PathManager;
pub use process::{is_another_instance_running, stop_tts_processes};
