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
use alloc::vec::Vec;
use super::decoder::{AudioInfo, Decoder};
use super::wav_pcm::{decode_sample, parse_riff};

pub struct WavDecoder {
    bytes: Vec<u8>,
    data_off: usize,
    data_len: usize,
    rate: u32,
    channels: u8,
    bits: u16,
    pos: usize,
}

impl WavDecoder {
    pub fn new(bytes: Vec<u8>) -> Result<WavDecoder, &'static str> {
        let (rate, channels, bits, data_off, data_len) = parse_riff(&bytes)?;
        Ok(WavDecoder { bytes, data_off, data_len, rate, channels, bits, pos: 0 })
    }
}

impl Decoder for WavDecoder {
    fn info(&self) -> AudioInfo {
        let block_align = (self.bits / 8) as usize * self.channels as usize;
        AudioInfo {
            rate: self.rate,
            channels: self.channels,
            total_frames: Some((self.data_len / block_align) as u64),
        }
    }

    fn next(&mut self, out: &mut [i16]) -> usize {
        let bytes_per_sample = (self.bits / 8) as usize;
        let total_samples = self.data_len / bytes_per_sample;
        let mut written = 0;
        while written < out.len() && self.pos < total_samples {
            let byte_off = self.data_off + self.pos * bytes_per_sample;
            out[written] = decode_sample(&self.bytes[byte_off..], self.bits);
            self.pos += 1;
            written += 1;
        }
        written
    }

    fn seek(&mut self, frame: u64) -> bool {
        let bytes_per_sample = (self.bits / 8) as usize;
        if bytes_per_sample == 0 || self.channels == 0 {
            return false;
        }
        let total_samples = self.data_len / bytes_per_sample;
        let sample = frame.saturating_mul(self.channels as u64);
        self.pos = (sample as usize).min(total_samples);
        true
    }
}
