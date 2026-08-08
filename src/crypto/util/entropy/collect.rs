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

use super::super::rng::{fill_random_bytes, get_random_bytes};
use super::hardware::{hardware_entropy64, hardware_random64};
use crate::crypto::hash::sha256;

pub fn gather_entropy() -> [u8; 32] {
    let mut entropy = [0u8; 32];
    let mut offset = 0;

    for _ in 0..4 {
        if let Some(val) = hardware_entropy64() {
            if offset + 8 <= entropy.len() {
                entropy[offset..offset + 8].copy_from_slice(&val.to_ne_bytes());
                offset += 8;
            }
        }
    }

    while offset < entropy.len() {
        if let Some(val) = hardware_random64() {
            let remaining = entropy.len() - offset;
            let copy_len = core::cmp::min(8, remaining);
            entropy[offset..offset + copy_len].copy_from_slice(&val.to_ne_bytes()[..copy_len]);
            offset += copy_len;
        } else {
            break;
        }
    }

    if offset == 0 {
        fill_random_bytes(&mut entropy);
        return entropy;
    }

    // The cycle counter only stirs the pool; it is not the entropy source.
    let ticks = crate::arch::read_time_counter();
    let timestamp = crate::time::timestamp_millis();

    let mut mixer = Vec::with_capacity(entropy.len() + 16);
    mixer.extend_from_slice(&entropy);
    mixer.extend_from_slice(&ticks.to_ne_bytes());
    mixer.extend_from_slice(&timestamp.to_ne_bytes());

    let hash = sha256(&mixer);
    entropy.copy_from_slice(&hash);

    entropy
}

pub fn get_entropy(len: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(len);
    let mut filled = 0;

    while filled < len {
        let chunk = gather_entropy();
        let remaining = len - filled;
        let copy_len = core::cmp::min(chunk.len(), remaining);

        result.extend_from_slice(&chunk[..copy_len]);
        filled += copy_len;
    }

    result
}

pub fn fill_entropy(buf: &mut [u8]) {
    let mut offset = 0;

    while offset < buf.len() {
        let chunk = gather_entropy();
        let remaining = buf.len() - offset;
        let copy_len = core::cmp::min(chunk.len(), remaining);

        buf[offset..offset + copy_len].copy_from_slice(&chunk[..copy_len]);
        offset += copy_len;
    }
}

pub fn get_random_u64() -> u64 {
    if let Some(val) = hardware_entropy64() {
        val
    } else if let Some(val) = hardware_random64() {
        val
    } else {
        let bytes = get_random_bytes();
        u64::from_ne_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }
}

pub fn fill_random(buffer: &mut [u8]) -> Result<(), &'static str> {
    let entropy = get_entropy(buffer.len());
    buffer.copy_from_slice(&entropy[..buffer.len()]);
    Ok(())
}

pub fn rand_u32() -> u32 {
    let entropy = get_entropy(4);
    u32::from_le_bytes([entropy[0], entropy[1], entropy[2], entropy[3]])
}

pub fn rand_u64() -> u64 {
    let entropy = get_entropy(8);
    u64::from_le_bytes([
        entropy[0], entropy[1], entropy[2], entropy[3], entropy[4], entropy[5], entropy[6],
        entropy[7],
    ])
}
