//! Définition de l'interface en ligne de commande
//!
//! Utilise clap pour parser les arguments CLI.

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "gsp",
    version,
    about = "Lecteur d'écran avec détection automatique de langue",
    long_about = "GSP lit du texte depuis la sélection, le presse-papier, un fichier ou une image OCR"
)]
pub struct Args {
    /// Source du texte à lire
    #[arg(
        short = 's',
        long,
        help = "Source du texte: selection, clipboard, file, stdin",
        default_value = "clipboard",
        value_parser = ["selection", "clipboard", "file", "stdin"]
    )]
    pub source: String,

    /// Chemin du fichier (requis si source=file)
    #[arg(
        short = 'f',
        long,
        help = "Chemin du fichier à lire (pour source=file)"
    )]
    pub file: Option<String>,

    /// Moteur TTS à utiliser
    #[arg(
        short = 'y',
        long = "tts",
        help = "Moteur TTS: pico, espeak, espeak-ng",
        default_value = "pico",
        value_parser = ["pico", "espeak", "espeak-ng"]
    )]
    pub engine_tts: String,

    /// Langue cible pour la synthèse vocale
    #[arg(
        short = 'l',
        long = "lang",
        help = "Langue cible pour le TTS",
        default_value = "fr-FR",
        value_parser = [
            "fr-FR", "en-US", "en-GB", "de-DE", "es-ES", "it-IT",
            "pt-PT", "nl-NL", "pl-PL", "ru-RU", "ja-JP", "zh-CN"
        ]
    )]
    pub lang_targets: String,

    /// Vitesse de lecture
    #[arg(
        long,
        help = "Vitesse de lecture (0.5 à 2.0)",
        default_value = "1.0"
    )]
    pub speed: String,

    /// Activer la traduction
    #[arg(
        short = 'T',
        long = "translate",
        help = "Activer la traduction automatique vers la langue cible",
        action = ArgAction::SetTrue
    )]
    pub translate: bool,

    /// Arrêter les processus TTS
    #[arg(
        short = 'p',
        long,
        help = "Arrêter les processus TTS en cours",
        action = ArgAction::SetTrue
    )]
    pub stop: bool,

    /// Mode verbose
    #[arg(
        short = 'v',
        long,
        help = "Mode verbose (logs détaillés)",
        action = ArgAction::SetTrue
    )]
    pub verbose: bool,
}
