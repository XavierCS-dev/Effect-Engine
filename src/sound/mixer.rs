use anyhow::Result;
use rodio::{
    queue::SourcesQueueOutput, source::SamplesConverter, Decoder, OutputStream, OutputStreamHandle,
    Sink, Source,
};
use std::{collections::HashMap, fs::*, io::*};

use crate::engine::util::effect_error::EffectError;

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct AudioID(pub &'static str);

struct AudioTrack {
    sink: Option<Sink>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

pub struct Mixer {
    tracks: HashMap<AudioID, AudioTrack>,
    effects: HashMap<AudioID, AudioTrack>,
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
    pub fn create_sink(mixer: &mut Mixer, path: &'static str) -> Result<AudioTrack> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file).unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut track = AudioTrack {
            sink: None,
            _stream,
            stream_handle,
        };
        let sink = Sink::try_new(&track.stream_handle).unwrap();
        sink.append(source);
        sink.pause();
        track.sink = Some(sink);
        Ok(track)
    }

    pub fn add_track(mixer: &mut Mixer, id: AudioID, path: &'static str) -> Result<()> {
        let sink = MixerSystem::create_sink(mixer, path)?;
        mixer.tracks.insert(id, sink);
        Ok(())
    }

    pub fn add_effect(mixer: &mut Mixer, id: AudioID, path: &'static str) -> Result<()> {
        let sink = MixerSystem::create_sink(mixer, path)?;
        mixer.effects.insert(id, sink);
        Ok(())
    }

    pub fn play_track(mixer: &Mixer, id: AudioID) -> Result<()> {
        let track = mixer
            .tracks
            .get(&id)
            .ok_or(EffectError::new("Track not in mixer"))?;
        track.sink.as_ref().unwrap().play();

        Ok(())
    }
}
