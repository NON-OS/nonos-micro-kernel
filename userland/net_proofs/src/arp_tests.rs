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

use crate::arp::packet::ArpPacket;

extern crate alloc;
use alloc::vec::Vec;

// ARP packets arrive from the link layer. Parsing must never panic on any input
// and must return None for anything that is not a well-formed ARP packet.

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

#[test]
fn arp_parse_never_panics_over_adversarial_input() {
    // The ARP-over-Ethernet/IPv4 packet is 28 bytes; sweep around that length.
    for &len in &[0usize, 1, 27, 28, 29, 30, 42, 64, 128] {
        for seed in 0..30_000u32 {
            let _ = ArpPacket::parse(&packet(len, seed));
        }
    }
}

#[test]
fn arp_rejects_truncated_packets() {
    // Anything shorter than a full ARP packet must be rejected, never parsed.
    for len in 0..28usize {
        assert!(ArpPacket::parse(&packet(len, len as u32)).is_none());
    }
}
