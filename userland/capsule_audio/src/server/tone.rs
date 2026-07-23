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

const SAMPLE_RATE: u32 = 48_000;

pub fn synth(freq_hz: u32, ms: u32, gain_q15: u16, out: &mut [i16]) {
    let frames = out.len() / 2;
    let fill = ((ms as usize * SAMPLE_RATE as usize) / 1000).min(frames);
    let half = if freq_hz == 0 {
        frames.max(1)
    } else {
        (SAMPLE_RATE / (2 * freq_hz)).max(1) as usize
    };
    let amp = gain_q15 as i16;
    let mut i = 0usize;
    while i < fill {
        let s = if (i / half) & 1 == 0 { amp } else { -amp };
        out[i * 2] = s;
        out[i * 2 + 1] = s;
        i += 1;
    }
    while i < frames {
        out[i * 2] = 0;
        out[i * 2 + 1] = 0;
        i += 1;
    }
}
