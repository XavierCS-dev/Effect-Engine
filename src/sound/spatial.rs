use std::io::Cursor;

use rodio::{OutputStream, OutputStreamHandle, SpatialSink};

use crate::engine::primitives::vector::Vector3;

pub struct SpatialAudioTrack {
    sink: Option<SpatialSink>,
    data: Cursor<Vec<u8>>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    position: Vector3<f32>,
}
