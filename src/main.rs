//! GSP - Screen Reader avec détection automatique de langue
//!
//! Application de lecture d'écran avec support TTS, OCR et traduction.

mod cli;
mod config;
mod engine;
mod error;
mod logging;
mod pipeline;
mod utils;

use clap::Parser;
use cli::Args;
use config::Config;
use engine::{TtsFactory, ClipboardInput, SelectionInput, StdinInput, FileInput, RodioPlayer, LibreTranslate};
use pipeline::ProcessingPipeline;
use utils::{is_another_instance_running, stop_tts_processes, Language};
use error::{Result, GspError};
use tracing::{info, error, debug};

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Charger la configuration
    let config = Config::load().unwrap_or_default();
    
    // Initialiser le logging
    logging::init(config.paths.log_dir.clone(), args.verbose)?;
    
    info!("Démarrage de GSP v{}", env!("CARGO_PKG_VERSION"));
    debug!("Configuration: {:?}", config);
    
    // Gestion de l'arrêt
    if args.stop {
        info!("Arrêt des processus TTS en cours");
        stop_tts_processes();
        return Ok(());
    }
    
    // Vérifier les instances multiples
    if is_another_instance_running() {
        error!("Une autre instance de GSP est déjà en cours d'exécution");
        return Err(GspError::InstanceAlreadyRunning);
    }
    
    // Sélectionner la source d'entrée
    let input: Box<dyn engine::InputEngine> = match args.source.as_str() {
        "clipboard" => Box::new(ClipboardInput::new()),
        "selection" => Box::new(SelectionInput::new()),
        "stdin" => Box::new(StdinInput::new()),
        "file" => {
            if let Some(ref path) = args.file {
                Box::new(FileInput::new(path))
            } else {
                return Err(GspError::ConfigError("Option --file requise pour la source 'file'".to_string()));
            }
        }
        _ => Box::new(ClipboardInput::new()),
    };
    
    // Créer le moteur TTS
    let tts = TtsFactory::create(&args.engine_tts, &args.lang_targets)?;
    
    // Créer le lecteur audio
    let mut audio = RodioPlayer::new();
    audio.set_volume(config.audio.volume);
    
    // Créer le pipeline
    let mut pipeline = ProcessingPipeline::new(input, tts, audio)
        .with_lang(&args.lang_targets)
        .with_speed(args.speed.parse().unwrap_or(1.0));
    
    // Ajouter la traduction si demandée
    if args.translate {
        let translator = LibreTranslate::new();
        pipeline = pipeline.with_translation(translator);
    }
    
    // Exécuter le pipeline
    pipeline.execute()?;
    
    Ok(())
}
