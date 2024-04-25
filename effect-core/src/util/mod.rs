pub mod effect_error;

use std::fs;
use std::io::prelude::*;

pub fn file_to_bytes(path: &str) -> Vec<u8> {
    let mut file_bytes: Vec<u8> = Vec::new();
    let mut file = fs::File::open(path).expect(format!("Could not find file {path}").as_str());
    file.read_to_end(&mut file_bytes).unwrap();
    file_bytes
}
