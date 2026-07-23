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

pub const FRAMES: usize = 1024;
pub const SAMPLES: usize = FRAMES * 2;
pub const BYTES: usize = SAMPLES * 2;

pub struct Mixer {
    acc: [i16; SAMPLES],
}

impl Mixer {
    pub fn new() -> Self {
        Mixer { acc: [0i16; SAMPLES] }
    }

    pub fn add(&mut self, src: &[i16]) {
        let n = src.len().min(SAMPLES);
        let mut i = 0;
        while i < n {
            self.acc[i] = self.acc[i].saturating_add(src[i]);
            i += 1;
        }
    }

    pub fn clear(&mut self) {
        let mut i = 0;
        while i < SAMPLES {
            self.acc[i] = 0;
            i += 1;
        }
    }

    pub fn write_bytes(&self, out: &mut [u8]) {
        let n = (out.len() / 2).min(SAMPLES);
        let mut i = 0;
        while i < n {
            let le = self.acc[i].to_le_bytes();
            out[i * 2] = le[0];
            out[i * 2 + 1] = le[1];
            i += 1;
        }
    }
}
