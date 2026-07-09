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

use crate::dhcp::constants::{HEADER_LEN, MAGIC_COOKIE};
use crate::dhcp::parse;

extern crate alloc;
use alloc::vec::Vec;

// DHCP replies come from the network and carry a variable-length option list
// (type/length/value). Parsing must never panic, and in particular an option
// whose length field runs past the end of the packet must be rejected, not read
// out of bounds.

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

fn message(len: usize, seed: u32) -> Vec<u8> {
    let mut s = seed | 1;
    (0..len).map(|_| (xorshift(&mut s) & 0xff) as u8).collect()
}

// A well-formed fixed header with the magic cookie, followed by adversarial
// option bytes, to drive the option parser with hostile type/length values.
fn with_options(seed: u32, option_bytes: usize) -> Vec<u8> {
    let mut m = message(HEADER_LEN + option_bytes, seed);
    // Magic cookie occupies the four bytes ending the fixed header.
    m[HEADER_LEN - 4..HEADER_LEN].copy_from_slice(&MAGIC_COOKIE);
    m
}

#[test]
fn dhcp_parse_never_panics_over_adversarial_messages() {
    for &len in &[0usize, 1, 100, HEADER_LEN - 1, HEADER_LEN, HEADER_LEN + 1, 300, 512, 1024] {
        for seed in 0..15_000u32 {
            let _ = parse(&message(len, seed));
        }
    }
}

#[test]
fn dhcp_option_lengths_never_read_out_of_bounds() {
    // Valid cookie so the option parser actually runs, with adversarial option
    // type/length/value bytes (including lengths that claim more than remains).
    for &opts in &[0usize, 1, 2, 3, 4, 8, 16, 64, 256] {
        for seed in 0..20_000u32 {
            let _ = parse(&with_options(seed, opts));
        }
    }
    // Explicit oversized-length option: type=53, length=0xFF, but no value.
    let mut m = with_options(1, 0);
    m.push(53);
    m.push(0xFF);
    let _ = parse(&m);
}
