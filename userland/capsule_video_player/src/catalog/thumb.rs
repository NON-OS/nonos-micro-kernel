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


use alloc::vec;
use alloc::vec::Vec;

use crate::player::decode::FrameDecoder;

pub const THUMB_W: u32 = 192;
pub const THUMB_H: u32 = 108;
const LIST: &[u8; 4] = b"LIST";
const VIDEO_CHUNKS: [&[u8; 4]; 2] = [b"00dc", b"00db"];
const SOI: [u8; 2] = [0xff, 0xd8];

pub fn extract(head: &[u8], w: u32, h: u32) -> Option<Vec<u32>> {
    let jpeg = first_frame(head)?;
    let pixels = (w as usize).checked_mul(h as usize)?;
    let mut full = vec![0u32; pixels];
    FrameDecoder::new().decode(jpeg, w, h, &mut full).ok()?;
    Some(downsample(&full, w, h))
}

fn first_frame(head: &[u8]) -> Option<&[u8]> {
    let mut at = 12;
    while at + 8 <= head.len() {
        let tag = &head[at..at + 4];
        let size = u32::from_le_bytes(head[at + 4..at + 8].try_into().ok()?) as usize;
        if tag == LIST {
            at += 12;
            continue;
        }
        if VIDEO_CHUNKS.iter().any(|c| *c == tag) {
            let body = head.get(at + 8..at + 8 + size)?;
            return body.starts_with(&SOI).then_some(body);
        }
        at += 8 + size + (size & 1);
    }
    None
}

fn fit(w: u32, h: u32) -> (u32, u32) {
    if w * THUMB_H >= h * THUMB_W {
        (THUMB_W, (THUMB_W * h / w).clamp(1, THUMB_H))
    } else {
        ((THUMB_H * w / h).clamp(1, THUMB_W), THUMB_H)
    }
}

fn downsample(full: &[u32], w: u32, h: u32) -> Vec<u32> {
    let mut out = vec![0u32; (THUMB_W * THUMB_H) as usize];
    let (fw, fh) = fit(w, h);
    let (ox, oy) = ((THUMB_W - fw) / 2, (THUMB_H - fh) / 2);
    for y in 0..fh {
        let sy = (y * h / fh).min(h - 1) as usize;
        for x in 0..fw {
            let sx = (x * w / fw).min(w - 1) as usize;
            out[((y + oy) * THUMB_W + x + ox) as usize] = full[sy * w as usize + sx];
        }
    }
    out
}
