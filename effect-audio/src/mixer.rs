use anyhow::Result;
use effect_util::effect_error::EffectError;
use rodio::{
    queue::SourcesQueueOutput, source::SamplesConverter, Decoder, OutputStream, OutputStreamHandle,
    Sink, Source, SpatialSink,
};
use std::{collections::HashMap, fs::*, io::*, time::Duration};

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

    pub fn track_volume(&self, id: AudioID) -> Result<f32> {
        let track = self
            .tracks
            .get(&id)
            .ok_or(EffectError::new("Track not in mixer"))?;
        Ok(track.sink.as_ref().unwrap().volume())
    }

    pub fn track_speed(&self, id: AudioID) -> Result<f32> {
        let track = self
            .tracks
            .get(&id)
            .ok_or(EffectError::new("Track not in mixer"))?;
        Ok(track.sink.as_ref().unwrap().speed())
    }

    pub fn is_paused(&self, id: AudioID) -> Result<bool> {
        let track = self
            .tracks
            .get(&id)
            .ok_or(EffectError::new("Track not in mixer"))?;
        Ok(track.sink.as_ref().unwrap().is_paused())
    }
}

pub struct MixerSystem;

impl MixerSystem {
    pub fn create_sink(
        path: &'static str,
        is_track: bool,
        repeat_infinite: bool,
        starting_point: Duration,
    ) -> Result<AudioTrack> {
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
        if is_track {
            let sink = Sink::try_new(&track.stream_handle).unwrap();
            if repeat_infinite {
                let source = Decoder::new(track.data.clone())
                    .unwrap()
                    .repeat_infinite()
                    .skip_duration(starting_point);
                sink.append(source);
            } else {
                let source = Decoder::new(track.data.clone())
                    .unwrap()
                    .skip_duration(starting_point);
                sink.append(source);
            }
            sink.pause();
            track.sink = Some(sink);
        }
        Ok(track)
    }

    /// Tracks have much more versatility in terms of their playback controls,
    /// however they must be manually reset to start at the beginning.
    pub fn add_track(
        mixer: &mut Mixer,
        id: AudioID,
        path: &'static str,
        starting_point: Duration,
        repeat_infinite: bool,
    ) -> Result<()> {
        let sink = MixerSystem::create_sink(path, true, repeat_infinite, starting_point)?;
        mixer.tracks.insert(id, sink);
        Ok(())
    }

    /// Effects can be replayed as many times as you like without reset
    /// There is a performance penality for this, however it is smaller for short effects.
    pub fn add_effect(mixer: &mut Mixer, id: AudioID, path: &'static str) -> Result<()> {
        let sink = MixerSystem::create_sink(path, true, false, Duration::from_secs(0))?;
        mixer.effects.insert(id, sink);
        Ok(())
    }

    pub fn play_effect_controlled(
        mixer: &Mixer,
        id: AudioID,
        speed: f32,
        volume: f32,
    ) -> Result<()> {
        let effect = mixer
            .effects
            .get(&id)
            .ok_or(EffectError::new("Effect not in mixer"))?;
        let sink = Sink::try_new(&effect.stream_handle).unwrap();
        sink.set_volume(volume);
        sink.set_speed(speed);
        let source = Decoder::new(effect.data.clone()).unwrap().repeat_infinite();
        sink.append(source);
        sink.detach();
        Ok(())
    }

    pub fn play_effect(mixer: &Mixer, id: AudioID) -> Result<()> {
        let effect = mixer
            .effects
            .get(&id)
            .ok_or(EffectError::new("Effect not in mixer"))?;
        let sink = Sink::try_new(&effect.stream_handle).unwrap();
        let source = Decoder::new(effect.data.clone()).unwrap().repeat_infinite();
        sink.append(source);
        sink.detach();
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

    pub fn reset_track(
        mixer: &mut Mixer,
        id: AudioID,
        starting_point: Duration,
        repeat_infinite: bool,
    ) -> Result<()> {
        let track = mixer
            .tracks
            .get_mut(&id)
            .ok_or(EffectError::new("Track not in mixer"))?;
        let sink = track.sink.as_ref().unwrap();
        sink.pause();
        sink.clear();
        if repeat_infinite {
            let source = Decoder::new(track.data.clone())
                .unwrap()
                .repeat_infinite()
                .skip_duration(starting_point);
            sink.append(source);
        } else {
            let source = Decoder::new(track.data.clone())
                .unwrap()
                .skip_duration(starting_point);
            sink.append(source);
        }
        sink.pause();
        Ok(())
    }

    pub fn set_track_speed(mixer: &Mixer, id: AudioID, speed: f32) -> Result<()> {
        let track = mixer
            .tracks
            .get(&id)
            .ok_or(EffectError::new("Track not in mixer"))?;
        track.sink.as_ref().unwrap().set_speed(speed);
        Ok(())
    }

    pub fn set_track_volume(mixer: &Mixer, id: AudioID, volume: f32) -> Result<()> {
        let track = mixer
            .tracks
            .get(&id)
            .ok_or(EffectError::new("Track not in mixer"))?;
        track.sink.as_ref().unwrap().set_volume(volume);
        Ok(())
    }

    pub fn remove_track(mixer: &mut Mixer, id: AudioID) {
        let _ = mixer.tracks.remove(&id);
    }

    pub fn remove_effect(mixer: &mut Mixer, id: AudioID) {
        let _ = mixer.effects.remove(&id);
    }
}
