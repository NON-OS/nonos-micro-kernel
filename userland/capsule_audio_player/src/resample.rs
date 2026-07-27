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

#[path = "resample/interp.rs"]
mod interp;

extern crate alloc;
use alloc::vec::Vec;
use interp::advance;

pub const OUT_RATE: u32 = 48_000;

pub struct Resampler {
    src_rate: u32,
    channels: u8,
    phase: u64,
    prev_l: i16,
    prev_r: i16,
    started: bool,
}

impl Resampler {
    pub fn new(src_rate: u32, src_channels: u8) -> Self {
        Self {
            src_rate,
            channels: src_channels.max(1),
            phase: 0,
            prev_l: 0,
            prev_r: 0,
            started: false,
        }
    }

    pub fn process(&mut self, src: &[i16], out: &mut Vec<i16>) {
        advance(self, src, out);
    }
}
