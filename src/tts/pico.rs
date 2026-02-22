use super::TtsEgine;
use crate::utils::textutils::replace;
use std::process::{Command, Stdio};
use tracing::{debug, error, info};

fn pico_effect(text: &str, speed: i32, pitch: i32, volume: i32) -> String {
    let speed = format!("<speed level=\"{}\">{}</speed>", speed, text);
    let pitch = format!("<pitch level=\"{}\">{}</pitch>", pitch, speed);
    let volume = format!("<volume level=\"{}\">{}</volume>", volume, pitch);
    volume
}

pub struct Pico {
    pub lang: String,
    pub speed: i32,
    pub output_file: String,
}

impl Default for Pico {
    fn default() -> Self {
        Self {
            lang: "fr-FR".to_string(),
            speed: 1,
            output_file: "/dev/shm/out.wav".to_string(),
        }
    }
}

impl Pico {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TtsEgine for Pico {
    fn speak(&self, text: &str) {
        let text = replace(text);
        let effect = pico_effect(&text, self.speed, 100, 120);

        debug!(
            "Exécution de pico2wave: lang={}, output={}, texte={} caractères",
            self.lang, self.output_file, text.len()
        );

        match Command::new("pico2wave")
            .arg(format!("--lang={}", self.lang))
            .arg(format!("-w={}", self.output_file))
            .arg("--")
            .arg(effect)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    info!("Génération audio réussie avec pico2wave");
                    debug!("Fichier généré: {}", self.output_file);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    error!("pico2wave a échoué: {}", stderr);
                }
            }
            Err(e) => {
                error!("Impossible d'exécuter pico2wave: {}", e);
            }
        }
    }

    fn set_lang(&mut self, lang: String) -> &mut Self {
        self.lang = lang;
        self
    }

    fn set_speed(&mut self, speed: i32) -> &mut Self {
        self.speed = speed;
        self
    }
}
