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

use super::defs::{Fed, State};
use super::machine::{Transport, DECODE_FRAMES};

const FEED_FRAMES: usize = 1024;

impl Transport {
    pub fn pump(&mut self) {
        if self.state != State::Playing {
            return;
        }
        loop {
            if self.scratch_out.is_empty() {
                let ch = self.scratch_src.len() / DECODE_FRAMES;
                let want = DECODE_FRAMES * ch;
                let got = match self.decoder.as_mut() {
                    Some(d) => d.next(&mut self.scratch_src[..want]),
                    None => return,
                };
                if got == 0 {
                    self.state = State::Stopped;
                    crate::mark::mark("[PLAYER] eof\n");
                    return;
                }
                self.pos_frames += (got / ch) as u64;
                self.resampler.process(&self.scratch_src[..got], &mut self.scratch_out);
                apply_volume(&mut self.scratch_out, self.volume_q15);
            }
            let n = core::cmp::min(FEED_FRAMES * 2, self.scratch_out.len());
            match self.client.feed(&self.scratch_out[..n]) {
                Fed::Accepted => {
                    self.scratch_out.drain(0..n);
                }
                Fed::WouldBlock => return,
            }
        }
    }
}

fn apply_volume(buf: &mut [i16], vol_q15: i32) {
    for s in buf.iter_mut() {
        let scaled = ((*s as i32 * vol_q15) >> 15).clamp(i16::MIN as i32, i16::MAX as i32);
        *s = scaled as i16;
    }
}
