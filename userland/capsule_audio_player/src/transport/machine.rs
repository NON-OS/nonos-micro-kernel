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
use crate::decode::Decoder;
use crate::resample::{Resampler, OUT_RATE};
use super::defs::{FeedSink, State};

const STREAM_FORMAT: u16 = 0;
pub(super) const DECODE_FRAMES: usize = 512;

pub struct Transport {
    pub(super) state: State, pub(super) pos_frames: u64, pub(super) dur_frames: u64,
    pub(super) volume_q15: i32, pub(super) decoder: Option<Box<dyn Decoder>>,
    pub(super) resampler: Resampler, pub(super) client: Box<dyn FeedSink>,
    pub(super) scratch_src: Vec<i16>, pub(super) scratch_out: Vec<i16>,
}

impl Transport {
    pub fn new(client: Box<dyn FeedSink>) -> Self {
        Self {
            state: State::Stopped, pos_frames: 0, dur_frames: 0, volume_q15: 1 << 15,
            decoder: None, resampler: Resampler::new(OUT_RATE, 2), client,
            scratch_src: alloc::vec![0; DECODE_FRAMES * 2], scratch_out: Vec::new(),
        }
    }

    pub fn open(&mut self, dec: Box<dyn Decoder>) -> Result<(), &'static str> {
        let info = dec.info();
        self.client.open(STREAM_FORMAT)?;
        self.resampler = Resampler::new(info.rate, info.channels);
        self.dur_frames = info.total_frames.unwrap_or(0);
        self.pos_frames = 0;
        self.scratch_out.clear();
        self.scratch_src = alloc::vec![0; DECODE_FRAMES * info.channels.max(1) as usize];
        self.decoder = Some(dec);
        self.state = State::Stopped;
        Ok(())
    }

    pub fn play(&mut self) { if self.decoder.is_some() { self.state = State::Playing; } }
    pub fn pause(&mut self) { self.state = State::Paused; self.client.pause(); }
    pub fn stop(&mut self) {
        self.state = State::Stopped; self.pos_frames = 0;
        self.client.close(); self.decoder = None; self.scratch_out.clear();
    }
    pub fn seek_frames(&mut self, f: u64) { self.pos_frames = f.min(self.dur_frames); }
    pub fn set_volume(&mut self, q15: i32) { self.volume_q15 = q15.clamp(0, 1 << 15); }
    pub fn state(&self) -> State { self.state }
    pub fn pos_frames(&self) -> u64 { self.pos_frames }
    pub fn dur_frames(&self) -> u64 { self.dur_frames }
    pub fn volume_q15(&self) -> i32 { self.volume_q15 }
}
