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

use crate::state::reasm::Reasm;
use crate::tcp::parse::parse;

extern crate alloc;
use alloc::vec::Vec;

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

fn bytes(len: usize, seed: u32) -> Vec<u8> {
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

// The TCP segment parser must never panic and the returned payload must stay
// inside the segment buffer.
#[test]
fn tcp_parse_never_panics_and_payload_stays_in_bounds() {
    let src = [10u8, 0, 0, 1];
    let dst = [10u8, 0, 0, 2];
    for &len in &[0usize, 1, 19, 20, 21, 40, 60, 64, 128, 512] {
        for seed in 0..20_000u32 {
            let seg = bytes(len, seed);
            if let Ok((_header, payload)) = parse(&src, &dst, &seg) {
                assert!(within(&seg, payload), "TCP payload escaped the segment");
            }
        }
    }
}

// The out-of-order reassembly buffer must never panic on hostile segment
// streams (overlapping, wildly out-of-order, at the sequence-space wrap).
#[test]
fn reassembly_never_panics_on_hostile_segment_streams() {
    for seed in 0..5_000u32 {
        let mut s = seed | 1;
        let mut r = Reasm::new();
        let inserts = (xorshift(&mut s) % 80) as usize;
        for _ in 0..inserts {
            let seq = xorshift(&mut s);
            let dlen = (xorshift(&mut s) % 48) as usize;
            let data: Vec<u8> = (0..dlen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
            r.insert(seq, data);
        }
        // Drain at several arbitrary receive points.
        for _ in 0..4 {
            let _ = r.drain_contiguous(xorshift(&mut s));
        }
    }
}

// Contiguous segments reassemble in order; a gap stops the drain.
#[test]
fn reassembly_joins_contiguous_and_stops_at_gaps() {
    let mut r = Reasm::new();
    r.insert(100, alloc::vec![1, 2]);
    r.insert(102, alloc::vec![3, 4]);
    assert_eq!(r.drain_contiguous(100), alloc::vec![1, 2, 3, 4]);

    let mut g = Reasm::new();
    g.insert(100, alloc::vec![1, 2]);
    g.insert(103, alloc::vec![9]); // gap at 102
    assert_eq!(g.drain_contiguous(100), alloc::vec![1, 2]);
}
