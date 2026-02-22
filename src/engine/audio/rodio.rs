//! Lecteur audio utilisant Rodio
//!
//! Implémente la lecture audio via la crate rodio.

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use rodio::{Sink, OutputStream, source::Source};
use tracing::{debug, error};
use crate::engine::audio::trait_::AudioPlayer;
use crate::error::{Result, AudioError};

/// Lecteur audio basé sur Rodio
pub struct RodioPlayer {
    volume: f32,
    _stream: Option<OutputStream>,
    sink: Option<Sink>,
}

impl RodioPlayer {
    pub fn new() -> Self {
        Self {
            volume: 1.0,
            _stream: None,
            sink: None,
        }
    }
    
    fn init_audio(&mut self) -> Result<()> {
        if self._stream.is_none() {
            let (stream, stream_handle) = OutputStream::try_default()
                .map_err(|e| AudioError::DeviceUnavailable(format!("Aucun périphérique audio: {}", e)))?;
            
            let sink = Sink::try_new(&stream_handle)
                .map_err(|e| AudioError::PlaybackFailed(format!("Impossible de créer le sink: {}", e)))?;
            
            self._stream = Some(stream);
            self.sink = Some(sink);
        }
        Ok(())
    }
}

impl Default for RodioPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayer for RodioPlayer {
    fn name(&self) -> &'static str {
        "rodio"
    }
    
    fn play(&mut self, path: &Path) -> Result<()> {
        debug!("Lecture du fichier: {:?}", path);
        
        self.init_audio()?;
        
        let sink = self.sink.as_ref()
            .ok_or_else(|| AudioError::PlaybackFailed("Sink non initialisé".to_string()))?;
        
        let file = File::open(path)
            .map_err(|e| AudioError::DecodeError(format!("Impossible d'ouvrir le fichier: {}", e)))?;
        
        let source = BufReader::new(file);
        let source = rodio::Decoder::new(source)
            .map_err(|e| AudioError::DecodeError(format!("Échec du décodage: {}", e)))?;
        
        sink.set_volume(self.volume);
        sink.append(source);
        
        // Attendre la fin de la lecture
        sink.sleep_until_end();
        
        debug!("Lecture terminée");
        Ok(())
    }
    
    fn stop(&mut self) -> Result<()> {
        if let Some(ref sink) = self.sink {
            sink.stop();
            debug!("Lecture arrêtée");
        }
        Ok(())
    }
    
    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(ref sink) = self.sink {
            sink.set_volume(self.volume);
        }
    }
}
