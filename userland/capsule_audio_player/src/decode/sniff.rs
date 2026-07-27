// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use super::decoder::Decoder;
use super::mp3::Mp3Decoder;
use super::wav::WavDecoder;

pub fn open(data: Vec<u8>) -> Result<Box<dyn Decoder>, &'static str> {
    if is_mp3(&data) {
        return Mp3Decoder::new(data).map(|d| Box::new(d) as Box<dyn Decoder>);
    }
    if is_wav(&data) {
        return WavDecoder::new(data).map(|d| Box::new(d) as Box<dyn Decoder>);
    }
    Err("unknown audio format")
}

fn is_mp3(data: &[u8]) -> bool {
    if data.starts_with(b"ID3") {
        return true;
    }
    data.len() >= 2 && data[0] == 0xFF && (data[1] & 0xE0) == 0xE0
}

fn is_wav(data: &[u8]) -> bool {
    data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WAVE"
}
