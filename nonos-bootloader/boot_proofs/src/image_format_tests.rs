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

use crate::image_format::footer::{FOOTER_MAGIC, FOOTER_SIZE, FOOTER_VERSION};
use crate::image_format::parse::parse_image_footer;

// The boot image footer is attacker-controlled: it names byte ranges for the
// kernel, signature and proof. The parser must never panic and must never
// return a slice that escapes the input buffer, no matter what the footer says.

fn within(data: &[u8], sub: &[u8]) -> bool {
    if sub.is_empty() {
        return true;
    }
    let d0 = data.as_ptr() as usize;
    let d1 = d0 + data.len();
    let s0 = sub.as_ptr() as usize;
    let s1 = s0 + sub.len();
    s0 >= d0 && s1 <= d1
}

fn assert_parse_safe(data: &[u8]) {
    if let Ok(p) = parse_image_footer(data) {
        assert!(within(data, p.kernel_bytes), "kernel slice escaped the buffer");
        assert!(within(data, p.signature_bytes), "signature slice escaped the buffer");
        if let Some(proof) = p.proof_bytes {
            assert!(within(data, proof), "proof slice escaped the buffer");
        }
    }
}

// Build an image whose trailing footer has a valid magic and version but
// adversarial region offsets/sizes derived from `seed`.
fn craft(len: usize, seed: u32) -> Vec<u8> {
    let mut x = seed | 1;
    let mut next = move || {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    };
    let mut d = vec![0u8; len];
    for b in d.iter_mut() {
        *b = (next() & 0xff) as u8;
    }
    let fs = len - FOOTER_SIZE;
    d[fs..fs + 8].copy_from_slice(&FOOTER_MAGIC);
    d[fs + 8..fs + 10].copy_from_slice(&FOOTER_VERSION.to_le_bytes());
    // Sweep hash/signature algorithm bytes across supported and unsupported.
    d[fs + 12] = (seed & 0x0f) as u8;
    d[fs + 13] = ((seed >> 4) & 0x0f) as u8;
    // total_image_size and the six region offset/size fields: fully adversarial.
    d[fs + 16..fs + 24].copy_from_slice(&(next() as u64).to_le_bytes());
    for off in [24usize, 28, 32, 36, 40, 44] {
        d[fs + off..fs + off + 4].copy_from_slice(&next().to_le_bytes());
    }
    d
}

#[test]
fn parse_never_panics_on_degenerate_input() {
    for d in [
        vec![],
        vec![0u8; 1],
        vec![0u8; FOOTER_SIZE - 1],
        vec![0u8; FOOTER_SIZE],
        vec![0xffu8; FOOTER_SIZE],
        vec![0u8; 4096],
        vec![0xffu8; 4096],
    ] {
        assert_parse_safe(&d);
    }
}

#[test]
fn parse_never_escapes_the_buffer_over_crafted_footers() {
    for &len in &[FOOTER_SIZE, FOOTER_SIZE + 1, 128, 256, 1024] {
        for seed in 0..25_000u32 {
            assert_parse_safe(&craft(len, seed));
        }
    }
    // A handful of large images to exercise wide offset ranges.
    for &len in &[8192usize, 65_536] {
        for seed in 0..500u32 {
            assert_parse_safe(&craft(len, seed));
        }
    }
}
