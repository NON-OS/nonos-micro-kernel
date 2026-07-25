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
use super::decoder::{AudioInfo, Decoder};
use super::minimp3_sys::mp3dec_t;
use super::mp3_frame::{decode_frame, empty_frame_info, new_decoder, MAX_SAMPLES};

pub struct Mp3Decoder {
    data: Vec<u8>,
    dec: Box<mp3dec_t>,
    cursor: usize,
    rate: u32,
    channels: u8,
    scratch: Vec<i16>,
    scratch_pos: usize,
}

impl Mp3Decoder {
    pub fn new(data: Vec<u8>) -> Result<Mp3Decoder, &'static str> {
        let mut dec = new_decoder();
        let mut pcm = [0i16; MAX_SAMPLES];
        let mut scratch = Vec::new();
        let mut cursor = 0usize;
        let mut rate = 0u32;
        let mut channels = 0u8;
        while cursor < data.len() {
            let mut fi = empty_frame_info();
            let ret = decode_frame(&mut dec, &data[cursor..], &mut pcm, &mut fi);
            cursor += fi.frame_bytes as usize;
            if ret > 0 {
                rate = fi.hz as u32;
                channels = fi.channels as u8;
                scratch.extend_from_slice(&pcm[..ret as usize * fi.channels as usize]);
                break;
            }
            if fi.frame_bytes == 0 {
                break;
            }
        }
        if rate == 0 || channels == 0 {
            return Err("no mp3 frame");
        }
        Ok(Mp3Decoder { data, dec, cursor, rate, channels, scratch, scratch_pos: 0 })
    }
}

impl Decoder for Mp3Decoder {
    fn info(&self) -> AudioInfo {
        AudioInfo { rate: self.rate, channels: self.channels, total_frames: None }
    }

    fn next(&mut self, out: &mut [i16]) -> usize {
        let mut pcm = [0i16; MAX_SAMPLES];
        let mut written = 0;
        while written < out.len() {
            if self.scratch_pos < self.scratch.len() {
                let take = (out.len() - written).min(self.scratch.len() - self.scratch_pos);
                out[written..written + take]
                    .copy_from_slice(&self.scratch[self.scratch_pos..self.scratch_pos + take]);
                self.scratch_pos += take;
                written += take;
                continue;
            }
            self.scratch.clear();
            self.scratch_pos = 0;
            if self.cursor >= self.data.len() {
                break;
            }
            let mut fi = empty_frame_info();
            let ret = decode_frame(&mut self.dec, &self.data[self.cursor..], &mut pcm, &mut fi);
            self.cursor += fi.frame_bytes as usize;
            if ret > 0 {
                self.scratch.extend_from_slice(&pcm[..ret as usize * fi.channels as usize]);
            } else if fi.frame_bytes == 0 {
                break;
            }
        }
        written
    }
}
