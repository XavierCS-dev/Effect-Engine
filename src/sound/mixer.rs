use anyhow::Result;
use rodio::{
    queue::SourcesQueueOutput, source::SamplesConverter, Decoder, OutputStream, OutputStreamHandle,
    Sink, Source,
};
use std::{collections::HashMap, fs::*, io::*};

use crate::engine::util::effect_error::EffectError;

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct AudioID(pub &'static str);

pub struct AudioTrack {
    sink: Option<Sink>,
    data: Cursor<Vec<u8>>,
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
    pub fn create_sink(path: &'static str, is_track: bool) -> Result<AudioTrack> {
        let mut file: Vec<u8> = Vec::new();
        File::open(path)?.read_to_end(&mut file)?;
        let cursor = Cursor::new(file);
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut track = AudioTrack {
            sink: None,
            _stream,
            stream_handle,
            data: cursor,
        };
        let sink = Sink::try_new(&track.stream_handle).unwrap();
        if is_track {
            let source = Decoder::new(track.data.clone()).unwrap().repeat_infinite();
            sink.append(source);
        } else {
            let source = Decoder::new(track.data.clone()).unwrap();
            sink.append(source);
        }
        sink.pause();
        track.sink = Some(sink);
        Ok(track)
    }

    pub fn add_track(mixer: &mut Mixer, id: AudioID, path: &'static str) -> Result<()> {
        let sink = MixerSystem::create_sink(path, true)?;
        mixer.tracks.insert(id, sink);
        Ok(())
    }

    pub fn add_effect(mixer: &mut Mixer, id: AudioID, path: &'static str) -> Result<()> {
        let sink = MixerSystem::create_sink(path, true)?;
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

    pub fn pause_track(mixer: &Mixer, id: AudioID) -> Result<()> {
        let track = mixer
            .tracks
            .get(&id)
            .ok_or(EffectError::new("Track not in mixer"))?;
        track.sink.as_ref().unwrap().pause();
        Ok(())
    }
}
