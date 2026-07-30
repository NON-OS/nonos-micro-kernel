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

use super::super::error::EntropyError;
use super::super::hardware::{cpu_entropy64, cpu_random64, read_cycle_counter};
use super::super::state::ENTROPY_COUNTER;
use super::get::get_entropy64;
use crate::drivers::virtio_rng;
use core::sync::atomic::Ordering;

pub fn collect_seed_entropy_secure() -> Result<[u8; 32], EntropyError> {
    let mut seed = [0u8; 32];
    if virtio_rng::is_available() {
        if virtio_rng::fill_random(&mut seed).is_ok() {
            return Ok(seed);
        }
    }
    // Count only bytes that came from a hardware entropy source. The TSC/stack
    // fallback below must never be accepted as a real seed.
    let mut hw_bytes = 0usize;
    let mut offset = 0;
    while offset < 32 {
        if let Some(v) = cpu_entropy64() {
            let len = core::cmp::min(8, 32 - offset);
            seed[offset..offset + len].copy_from_slice(&v.to_le_bytes()[..len]);
            offset += len;
            hw_bytes += len;
        } else {
            break;
        }
    }
    while offset < 32 {
        for _ in 0..10 {
            if let Some(v) = cpu_random64() {
                let len = core::cmp::min(8, 32 - offset);
                seed[offset..offset + len].copy_from_slice(&v.to_le_bytes()[..len]);
                offset += len;
                hw_bytes += len;
                break;
            }
            for _ in 0..50 {
                core::hint::spin_loop();
            }
        }
        if offset < 32 {
            let t1 = read_cycle_counter();
            for _ in 0..((t1 & 0x1F) + 1) {
                core::hint::spin_loop();
            }
            let t2 = read_cycle_counter();
            let len = core::cmp::min(8, 32 - offset);
            seed[offset..offset + len].copy_from_slice(&t2.wrapping_sub(t1).to_le_bytes()[..len]);
            offset += len;
        }
    }
    // Fail closed unless every seed byte came from hardware (virtio-rng returned
    // early above; rdseed/rdrand must supply all 32 here). Accepting a
    // TSC/stack-derived seed as "secure" would key the CSPRNG from low,
    // partly predictable entropy. init_rng propagates this Err and leaves the
    // RNG uninitialised rather than minting predictable keys and nonces.
    if hw_bytes < 32 {
        return Err(EntropyError::InsufficientEntropy);
    }
    let stack_addr = crate::arch::stack_pointer();
    let counter = ENTROPY_COUNTER.fetch_add(0xA7B3_C5D9_E1F4_2680, Ordering::SeqCst);
    let (sb, cb) = (stack_addr.to_le_bytes(), counter.to_le_bytes());
    for i in 0..8 {
        seed[i] ^= sb[i];
        seed[i + 8] ^= cb[i];
        seed[i + 16] ^= sb[7 - i];
        seed[i + 24] ^= cb[7 - i];
    }
    let tb = read_cycle_counter().to_le_bytes();
    for i in 0..8 {
        seed[i] ^= tb[i];
    }
    Ok(seed)
}

pub fn collect_seed_entropy() -> [u8; 32] {
    if let Ok(seed) = collect_seed_entropy_secure() {
        return seed;
    }
    let mut seed = [0u8; 32];
    for i in 0..4 {
        let entropy = get_entropy64();
        seed[i * 8..(i + 1) * 8].copy_from_slice(&entropy.to_le_bytes());
    }
    let stack_addr = crate::arch::stack_pointer();
    let sb = stack_addr.to_le_bytes();
    for i in 0..8 {
        seed[i] ^= sb[i];
    }
    seed
}

pub fn mix_entropy_into_seed(seed: &mut [u8; 32], additional: &[u8; 32]) {
    for i in 0..32 {
        seed[i] ^= additional[i];
    }
    let tb = read_cycle_counter().to_le_bytes();
    for i in 0..8 {
        seed[i] ^= tb[i];
        seed[i + 8] ^= tb[i];
        seed[i + 16] ^= tb[i];
        seed[i + 24] ^= tb[i];
    }
}
