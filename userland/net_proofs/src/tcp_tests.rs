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

// Ground-truth for the mandatory TCP checksum (RFC 793/1071). These catch the
// two real bugs a fuzz test cannot: omitting the IPv4 pseudo-header, and a fold
// that does not carry.
mod checksum_known_answer {
    use crate::tcp::checksum::compute;

    #[test]
    fn all_zero_segment_is_just_the_pseudo_header() {
        // src=dst=0.0.0.0, a 20-byte all-zero segment: the only contribution is
        // the pseudo-header's protocol (6) + tcp length (20) = 26, so the
        // checksum is the one's-complement of 26 = 0xFFE5. A checksum that
        // forgot the pseudo-header would return 0xFFFF here instead.
        assert_eq!(compute(&[0, 0, 0, 0], &[0, 0, 0, 0], &[0u8; 20]), 0xFFE5);
    }

    #[test]
    fn a_correct_checksum_verifies_to_zero_and_depends_on_the_addresses() {
        let src = [192, 168, 1, 10];
        let dst = [93, 184, 216, 34];
        // 20-byte TCP header (dst port 80, SYN), checksum field at 16..18 zero.
        let mut seg = alloc::vec![
            0x00, 0x50, 0xC1, 0x23, 0, 0, 0, 1, 0, 0, 0, 0, 0x50, 0x02, 0x72, 0x10, 0, 0, 0, 0,
        ];
        let ck = compute(&src, &dst, &seg);
        assert_ne!(ck, 0, "a real, non-trivial checksum");
        seg[16..18].copy_from_slice(&ck.to_be_bytes());
        assert_eq!(compute(&src, &dst, &seg), 0, "a valid internet checksum verifies to zero");
        // The pseudo-header is really folded in: a different source address
        // changes the checksum. If it did not, the pseudo-header was ignored.
        assert_ne!(compute(&[10, 0, 0, 1], &dst, &seg), 0);
    }
}
