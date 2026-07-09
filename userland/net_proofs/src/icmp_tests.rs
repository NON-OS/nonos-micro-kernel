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

use crate::icmp::parse::parse;

extern crate alloc;
use alloc::vec::Vec;

// ICMP packets arrive from the network. Parsing must never panic, and the
// returned payload slice must never escape the input buffer.

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

fn packet(len: usize, seed: u32) -> Vec<u8> {
    let mut s = seed | 1;
    (0..len).map(|_| (xorshift(&mut s) & 0xff) as u8).collect()
}

fn within(data: &[u8], sub: &[u8]) -> bool {
    if sub.is_empty() {
        return true;
    }
    let d0 = data.as_ptr() as usize;
    let s0 = sub.as_ptr() as usize;
    s0 >= d0 && s0 + sub.len() <= d0 + data.len()
}

#[test]
fn icmp_parse_never_panics_and_payload_stays_in_bounds() {
    for &len in &[0usize, 1, 7, 8, 9, 20, 28, 64, 128, 512] {
        for seed in 0..20_000u32 {
            let p = packet(len, seed);
            if let Ok((_header, payload)) = parse(&p) {
                assert!(within(&p, payload), "ICMP payload escaped the buffer");
            }
        }
    }
}
