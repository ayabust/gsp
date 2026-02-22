mod cli;
mod error;
mod input;
mod logging;
mod player;
mod translate;
mod tts;
mod utils;

use clap::Parser;
use cli::Args;
use error::{GspError, InputError, Result, TranslationError};
use input::Input;
use player::rodio::Rodio;
use tts::{espeak::Espeak, pico::Pico, Tts};
use utils::{get_pidof, textutils::*};

use crate::tts::espeakng::EspeakNg;

use std::path::PathBuf;
use tracing::{debug, error, info, warn};

fn main() {
    if let Err(e) = run() {
        error!("Erreur fatale: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Initialisation du logging
    let log_dir = args.log_dir.clone().map(PathBuf::from);
    logging::init(log_dir, args.verbose)?;

    info!("Démarrage de GSP v{}", env!("CARGO_PKG_VERSION"));
    debug!("Arguments: {:?}", args);

    if args.stop {
        info!("Arrêt de la lecture en cours");
        stop_tts();
        return Ok(());
    }

    if is_another_instance_running() {
        warn!("Une autre instance est déjà en cours d'exécution");
        return Err(GspError::InstanceAlreadyRunning);
    }

    debug!("Récupération du texte depuis la source: {}", args.source);
    let text = get_input_text(&args)?;
    debug!("Texte récupéré: {} caractères", text.len());

    let text = preprocess_text(&args, text);
    debug!("Texte prétraité: {} caractères", text.len());

    let translated_text = if let Some(ref lang_sources) = args.lang_sources {
        info!("Traduction depuis {} vers {}", lang_sources, args.lang_targets);
        translate_text(&args, lang_sources, text)?
    } else {
        text
    };

    debug!("Configuration du moteur TTS: {}", args.engine_tts);
    let mut tts = configure_tts(&args, translated_text);

    info!("Lecture du texte avec le moteur {}", args.engine_tts);
    match args.engine_tts.as_str() {
        "espeak" => tts.speak(&mut Espeak::new()),
        "espeak-ng" => tts.speak(&mut EspeakNg::new()),
        "pico" => tts.speak(&mut Pico::new()),
        _ => tts.speak(&mut Pico::new()),
    }
    .play(Rodio {});

    info!("Lecture terminée");
    Ok(())
}

fn stop_tts() {
    Tts::new().stop(Rodio {});
}

fn is_another_instance_running() -> bool {
    get_pidof("gsp").len() > 1
}

fn get_input_text(args: &Args) -> Result<String> {
    let text = Input::new(
        args.source.clone(),
        args.lang_sources
            .clone()
            .unwrap_or(args.lang_targets.clone()),
    )
    .input();

    if text.is_empty() {
        return Err(InputError::EmptyClipboard.into());
    }

    Ok(text)
}

fn preprocess_text(args: &Args, text: String) -> String {
    let mut text = if args.dev {
        read_vars(&text).to_lowercase()
    } else {
        text
    };

    text = parse_hashtag(&text);
    text = trim_whitespace(&text);
    text = remove_special_characters(&text);

    text
}

fn translate_text(args: &Args, lang_sources: &str, text: String) -> Result<String> {
    translate::Translate::new()
        .translate(
            args.engine_translation.as_str(),
            text.as_str(),
            lang_sources,
            args.lang_targets.as_str(),
        )
        .map_err(|e| TranslationError::TranslationFailed(e.to_string()).into())
}

fn configure_tts(args: &Args, text: String) -> Tts {
    let speed = args.speed.parse::<f32>().unwrap_or(1.0) * 100.0;
    let speed = speed as i32;

    let mut tts = Tts::new();
    tts.set_lang(args.lang_targets.to_string())
        .set_speed(speed)
        .set_text(text);

    tts
}
