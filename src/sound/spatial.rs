use std::{fs::*, io::*};

use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, SpatialSink};

use crate::engine::{camera::camera::Camera2D, primitives::vector::Vector3};

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
    pub fn calculate_position_2d(camera: Vector3<f32>, entity: Vector3<f32>) -> Vector3<f32> {
        Vector3::new(entity.x - camera.x, entity.y - camera.y, 0.0)
    }

    pub fn new_effect(position: Vector3<f32>, path: &'static str) -> Result<SpatialAudioEffect> {
        let mut file: Vec<u8> = Vec::new();
        File::open(path)?.read_to_end(&mut file)?;
        let cursor = Cursor::new(file);
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

    pub fn new_track(
        position: Vector3<f32>,
        path: &'static str,
        repeat_infinite: bool,
    ) -> Result<SpatialAudioTrack> {
        todo!()
    }

    pub fn play_track(track: &SpatialAudioTrack) -> Result<()> {
        todo!()
    }

    pub fn pause_track(track: &SpatialAudioTrack) -> Result<()> {
        todo!()
    }

    pub fn reset_track(track: &mut SpatialAudioTrack) -> Result<()> {
        todo!()
    }
}
