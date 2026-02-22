//! Gestion des processus système
//!
//! Utilitaires pour détecter et gérer les processus GSP.

use std::process::Command;
use tracing::debug;

/// Vérifie si une autre instance de GSP est en cours d'exécution
pub fn is_another_instance_running() -> bool {
    // Utiliser pgrep pour compter les instances
    match Command::new("pgrep").arg("-c").arg("gsp").output() {
        Ok(output) => {
            if let Ok(count) = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<u32>()
            {
                // Si plus d'une instance (celle-ci incluse), une autre tourne déjà
                let running = count > 1;
                debug!("Nombre d'instances GSP détectées: {}", count);
                running
            } else {
                false
            }
        }
        Err(_) => {
            // Fallback: essayer avec ps
            fallback_instance_check()
        }
    }
}

/// Vérification fallback avec ps
fn fallback_instance_check() -> bool {
    match Command::new("ps").args(["aux", "-o", "comm="]).output() {
        Ok(output) => {
            let ps_output = String::from_utf8_lossy(&output.stdout);
            let gsp_count = ps_output
                .lines()
                .filter(|line| line.contains("gsp"))
                .count();
            gsp_count > 1
        }
        Err(_) => false,
    }
}

/// Arrête tous les processus TTS en cours
pub fn stop_tts_processes() {
    let processes = ["espeak", "espeak-ng", "pico2wave", "spd-say"];

    for process in processes {
        let _ = Command::new("pkill").arg("-f").arg(process).output();
    }

    debug!("Processus TTS arrêtés");
}
