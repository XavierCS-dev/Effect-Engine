use std::{fs::*, io::*};

use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, SpatialSink};

use crate::engine::{camera::camera::Camera2D, primitives::vector::Vector3};

pub struct SpatialAudioEffect {
    data: Cursor<Vec<u8>>,
    position: Vector3<f32>,
    left_ear: [f32; 3],
    right_ear: [f32; 3],
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
pub struct SpatialAudio2DSystem;

impl SpatialAudio2DSystem {
    pub fn calculate_position(camera: Vector3<f32>, entity: Vector3<f32>) -> Vector3<f32> {
        Vector3::new(entity.x - camera.x, entity.y - camera.y, 0.0)
    }

    pub fn new_effect(position: Vector3<f32>, path: &'static str) -> Result<SpatialAudioEffect> {
        let mut file: Vec<u8> = Vec::new();
        File::open(path)?.read_to_end(&mut file)?;
        let cursor = Cursor::new(file);
        let left_ear = [-0.1, 0.0, 0.0];
        let right_ear = [0.1, 0.0, 0.0];
        Ok(SpatialAudioEffect {
            data: cursor,
            position,
            left_ear,
            right_ear,
        })
    }

    pub fn play_effect(effect: &SpatialAudioEffect) {
        todo!()
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
