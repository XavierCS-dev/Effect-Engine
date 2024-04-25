use std::{fs::*, io::*, time::Duration};

use anyhow::Result;
use effect_core::primitives::vector::Vector3;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source, SpatialSink};

pub struct SpatialAudioEffect {
    data: Cursor<Vec<u8>>,
    position: Vector3<f32>,
    left_ear: [f32; 3],
    right_ear: [f32; 3],
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

pub struct SpatialAudioTrack {
    data: Cursor<Vec<u8>>,
    sink: Option<SpatialSink>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    position: Vector3<f32>,
    left_ear: [f32; 3],
    right_ear: [f32; 3],
}

// depth should be ignored in 2D
// taking depth into account can cause issues with how the camera is positioned
pub struct SpatialAudioSystem;

impl SpatialAudioSystem {
    // Since camera z affects x and y coordinate ratios, find way to integerate the camera's
    // z coordinate to have consistent sptial audio distance
    pub fn calculate_position_2d(camera: Vector3<f32>, entity: Vector3<f32>) -> Vector3<f32> {
        Vector3::new(entity.x - camera.x, entity.y - camera.y, 0.0)
    }

    pub fn new_effect(position: Vector3<f32>, path: &'static str) -> Result<SpatialAudioEffect> {
        let mut file: Vec<u8> = Vec::new();
        File::open(path)?.read_to_end(&mut file)?;
        let cursor = Cursor::new(file);
        // Having this way around means the sound positions are correct
        let right_ear = [-0.1, 0.0, 0.0];
        let left_ear = [0.1, 0.0, 0.0];
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Ok(SpatialAudioEffect {
            data: cursor,
            position,
            left_ear,
            right_ear,
            _stream,
            stream_handle,
        })
    }

    pub fn play_effect(effect: &SpatialAudioEffect, volume: f32, speed: f32) {
        let sink = SpatialSink::try_new(
            &effect.stream_handle,
            [effect.position.x, effect.position.y, effect.position.z],
            effect.left_ear,
            effect.right_ear,
        )
        .unwrap();
        sink.set_volume(volume);
        sink.set_speed(speed);
        let source = Decoder::new(effect.data.clone()).unwrap();
        sink.append(source);
        sink.detach();
    }

    pub fn set_position_effect(effect: &mut SpatialAudioEffect, position: Vector3<f32>) {
        effect.position = position;
    }

    pub fn new_track(
        position: Vector3<f32>,
        path: &'static str,
        starting_point: Duration,
        repeat_infinite: bool,
    ) -> Result<SpatialAudioTrack> {
        let mut file: Vec<u8> = Vec::new();
        File::open(path)?.read_to_end(&mut file)?;
        let cursor = Cursor::new(file);
        // Having this way around means the sound positions are correct
        let right_ear = [-0.1, 0.0, 0.0];
        let left_ear = [0.1, 0.0, 0.0];
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut track = SpatialAudioTrack {
            data: cursor,
            sink: None,
            _stream,
            stream_handle,
            position,
            left_ear,
            right_ear,
        };
        let sink = SpatialSink::try_new(
            &track.stream_handle,
            [track.position.x, track.position.y, track.position.z],
            track.left_ear,
            track.right_ear,
        )
        .unwrap();
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
        Ok(track)
    }

    pub fn play_track(track: &SpatialAudioTrack) {
        track.sink.as_ref().unwrap().play();
    }

    pub fn pause_track(track: &SpatialAudioTrack) {
        track.sink.as_ref().unwrap().pause();
    }

    pub fn set_position_track(track: &mut SpatialAudioTrack, position: Vector3<f32>) {
        track.position = position;
    }

    pub fn reset_track(
        track: &mut SpatialAudioTrack,
        starting_point: Duration,
        repeat_infinite: bool,
    ) {
        let sink = track.sink.as_mut().unwrap();
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
    }
}
