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
use super::{Resampler, OUT_RATE};

const FRAC_BITS: u32 = 32;

pub(super) fn advance(r: &mut Resampler, src: &[i16], out: &mut Vec<i16>) {
    let ch = r.channels as usize;
    let n = src.len() / ch;
    if n == 0 {
        return;
    }
    if r.src_rate == OUT_RATE && r.channels == 2 {
        out.extend_from_slice(src);
        r.prev_l = src[(n - 1) * 2];
        r.prev_r = src[(n - 1) * 2 + 1];
        r.started = true;
        return;
    }
    let frame = |i: usize| -> (i16, i16) {
        let base = i * ch;
        let l = src[base];
        let rr = if ch >= 2 { src[base + 1] } else { l };
        (l, rr)
    };
    if !r.started {
        let (l0, r0) = frame(0);
        r.prev_l = l0;
        r.prev_r = r0;
        r.started = true;
    }
    let step = ((r.src_rate as u64) << FRAC_BITS) / (OUT_RATE as u64);
    let mut pos = r.phase;
    while (pos >> FRAC_BITS) < n as u64 {
        let idx = (pos >> FRAC_BITS) as usize;
        let frac = (pos & ((1u64 << FRAC_BITS) - 1)) as u32;
        let (al, ar) = if idx == 0 { (r.prev_l, r.prev_r) } else { frame(idx - 1) };
        let (bl, br) = frame(idx);
        out.push(lerp(al, bl, frac));
        out.push(lerp(ar, br, frac));
        pos += step;
    }
    r.phase = pos - ((n as u64) << FRAC_BITS);
    let (ll, lr) = frame(n - 1);
    r.prev_l = ll;
    r.prev_r = lr;
}

fn lerp(a: i16, b: i16, frac: u32) -> i16 {
    let diff = (b as i64 - a as i64) * (frac as i64);
    let rounded = (diff + (1i64 << (FRAC_BITS - 1))) >> FRAC_BITS;
    (a as i64 + rounded) as i16
}
