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

use crate::descriptors::hid_bindings;
use crate::descriptors::types::DT_CONFIGURATION;
use crate::protocol::MAX_HID_BINDINGS;

extern crate alloc;
use alloc::vec::Vec;

// A USB configuration descriptor is supplied by the device and is fully
// attacker-controlled. Parsing must never panic, must terminate (a zero-length
// descriptor must not loop forever), and must never return more bindings than
// the fixed cap.

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

// A well-formed configuration header (so the parser walks the descriptor list)
// followed by adversarial descriptor bytes.
fn craft(len: usize, seed: u32) -> Vec<u8> {
    let mut d = bytes(len.max(9), seed);
    d[0] = 9; // bLength of the config header
    d[1] = DT_CONFIGURATION;
    let total = d.len() as u16;
    d[2..4].copy_from_slice(&total.to_le_bytes());
    d
}

#[test]
fn hid_bindings_never_panics_over_adversarial_input() {
    for &len in &[0usize, 1, 8, 9, 10, 20, 64, 128, 256, 512] {
        for seed in 0..15_000u32 {
            let _ = hid_bindings(&bytes(len, seed));
        }
    }
}

#[test]
fn descriptor_walk_terminates_and_binding_count_is_bounded() {
    for &len in &[9usize, 10, 20, 64, 128, 256, 512] {
        for seed in 0..20_000u32 {
            if let Ok(bindings) = hid_bindings(&craft(len, seed)) {
                assert!(bindings.len() <= MAX_HID_BINDINGS, "too many bindings returned");
            }
        }
    }
    // A descriptor claiming bLength = 0 must be rejected, not looped on.
    let mut d = craft(9, 7);
    d.push(0); // bLength = 0
    d.push(0); // bDescriptorType
    let total = d.len() as u16;
    d[2..4].copy_from_slice(&total.to_le_bytes());
    let _ = hid_bindings(&d);
}
