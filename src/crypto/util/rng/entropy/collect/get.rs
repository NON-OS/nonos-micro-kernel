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
use crate::drivers::virtio_rng;
use core::sync::atomic::Ordering;

pub fn get_entropy64_secure() -> Result<u64, EntropyError> {
    if virtio_rng::is_available() {
        let mut buf = [0u8; 8];
        if virtio_rng::fill_random(&mut buf).is_ok() {
            return Ok(u64::from_le_bytes(buf));
        }
    }
    if let Some(v) = cpu_entropy64() {
        return Ok(v);
    }
    if let Some(v) = cpu_random64() {
        return Ok(v);
    }
    Err(EntropyError::HardwareFailure)
}

pub fn get_entropy64() -> u64 {
    if let Ok(v) = get_entropy64_secure() {
        return v;
    }
    crate::log_warn!(
        "[ENTROPY] All hardware entropy sources failed — falling back to emergency TSC-jitter mix"
    );
    emergency_entropy_mix()
}

#[cold]
pub(super) fn emergency_entropy_mix() -> u64 {
    let counter = ENTROPY_COUNTER.fetch_add(1, Ordering::SeqCst);
    let tsc1 = read_cycle_counter();
    let stack_addr = crate::arch::stack_pointer();
    for _ in 0..counter.wrapping_rem(16).wrapping_add(1) {
        core::hint::spin_loop();
    }
    let tsc2 = read_cycle_counter();
    let jitter = tsc2.wrapping_sub(tsc1);
    let mut state = counter;
    state = state.wrapping_add(0x9e3779b97f4a7c15);
    state ^= tsc1;
    state = (state ^ (state >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    state ^= stack_addr;
    state = (state ^ (state >> 27)).wrapping_mul(0x94d049bb133111eb);
    state ^= jitter;
    state ^= state >> 31;
    state
}

pub fn get_tsc_entropy() -> u64 {
    let counter = ENTROPY_COUNTER.fetch_add(1, Ordering::SeqCst);
    counter ^ read_cycle_counter()
}
