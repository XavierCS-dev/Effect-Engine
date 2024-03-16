use anyhow::Result;
use rodio::{
    queue::SourcesQueueOutput, source::SamplesConverter, Decoder, OutputStream, Sink, Source,
};
use std::{collections::HashMap, fs::*, io::*};

use crate::engine::util::effect_error::EffectError;

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct AudioID(pub &'static str);

pub struct Mixer {
    tracks: HashMap<AudioID, Sink>,
    effects: HashMap<AudioID, Sink>,
}

impl Mixer {
    pub fn new() -> Self {
        let tracks = HashMap::new();
        let effects = HashMap::new();
        Self { tracks, effects }
    }

    pub fn get_tracks(&self) -> Vec<&AudioID> {
        self.tracks.keys().collect()
    }

    pub fn get_effects(&self) -> Vec<&AudioID> {
        self.effects.keys().collect()
    }
}

pub struct MixerSystem;

impl MixerSystem {
    pub fn create_sink(path: &'static str) -> Result<Sink> {
        let mut data: Vec<u8> = Vec::new();
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file).unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        Ok(sink)
    }

    pub fn add_track(mixer: &mut Mixer, id: AudioID, path: &'static str) -> Result<()> {
        let sink = MixerSystem::create_sink(path)?;
        mixer.tracks.insert(id, sink);
        Ok(())
    }

    pub fn add_effect(mixer: &mut Mixer, id: AudioID, path: &'static str) -> Result<()> {
        let sink = MixerSystem::create_sink(path)?;
        mixer.effects.insert(id, sink);
        Ok(())
    }

    pub fn play_track(mixer: &Mixer, id: AudioID) -> Result<()> {
        let track = mixer
            .tracks
            .get(&id)
            .ok_or(EffectError::new("Track not in mixer"))?;
        track.play();

        Ok(())
    }
}
