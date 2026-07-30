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

use super::super::csprng::ChaChaRng;
use super::super::entropy::{
    collect_seed_entropy_secure, get_tsc_entropy, mark_bootloader_entropy_provided,
};
use super::super::error::{RngError, RngResult};
use super::init::{ensure_initialized, entropy_error_to_rng_error};
use super::state::{
    GLOBAL_RNG, GLOBAL_STATE, STATE_INITIALIZED, STATE_INITIALIZING, STATE_UNINITIALIZED,
};
use crate::crypto::util::constant_time::{compiler_fence, memory_fence};
use core::ptr;
use core::sync::atomic::Ordering;

pub fn seed_rng() -> RngResult<()> {
    ensure_initialized()?;

    let hw_seed = collect_seed_entropy_secure().map_err(entropy_error_to_rng_error)?;

    let mut combined = [0u8; 32];
    for i in 0..32 {
        combined[i] = hw_seed[i];
    }

    for i in 0..4 {
        let tsc = get_tsc_entropy();
        let offset = i * 8;
        let tsc_bytes = tsc.to_le_bytes();
        for j in 0..8 {
            combined[offset + j] ^= tsc_bytes[j];
        }
        for _ in 0..((tsc & 0xF) + 1) {
            core::hint::spin_loop();
        }
    }

    let stack_addr = crate::arch::stack_pointer();
    let stack_bytes = stack_addr.to_le_bytes();
    for i in 0..8 {
        combined[i] ^= stack_bytes[i];
        combined[i + 16] ^= stack_bytes[7 - i];
    }

    {
        let mut guard = GLOBAL_RNG.lock();
        if let Some(ref mut rng) = *guard {
            rng.reseed(combined);
        }
    }

    for b in &mut combined {
        unsafe { ptr::write_volatile(b, 0) };
    }
    compiler_fence();
    memory_fence();

    Ok(())
}

pub fn seed_from_bootloader(bootloader_entropy: &[u8; 32]) -> RngResult<()> {
    mark_bootloader_entropy_provided();

    let local_entropy = match collect_seed_entropy_secure() {
        Ok(seed) => seed,
        Err(_) => {
            let mut fallback = [0u8; 32];
            for i in 0..4 {
                let t1 = get_tsc_entropy();
                for _ in 0..((i * 11) + 7) {
                    core::hint::spin_loop();
                }
                let t2 = get_tsc_entropy();
                let jitter = t2.wrapping_sub(t1).wrapping_mul(0x9E3779B97F4A7C15);
                fallback[i * 8..(i + 1) * 8].copy_from_slice(&jitter.to_le_bytes());
            }
            fallback
        }
    };

    let mut combined = [0u8; 32];
    for i in 0..32 {
        combined[i] = bootloader_entropy[i] ^ local_entropy[i];
    }

    let rtc = crate::arch::wall_clock::unix_timestamp().unwrap_or(0);
    let rtc_bytes = rtc.to_le_bytes();
    for i in 0..8 {
        combined[i] ^= rtc_bytes[i];
        combined[i + 8] ^= rtc_bytes[7 - i];
        combined[i + 16] ^= rtc_bytes[i].wrapping_add(i as u8);
        combined[i + 24] ^= rtc_bytes[7 - i].wrapping_sub(i as u8);
    }

    let kernel_ms = crate::time::timestamp_millis();
    let ms_bytes = kernel_ms.to_le_bytes();
    for i in 0..8 {
        combined[i] ^= ms_bytes[i];
    }

    if GLOBAL_STATE.load(Ordering::Acquire) != STATE_INITIALIZED {
        match GLOBAL_STATE.compare_exchange(
            STATE_UNINITIALIZED,
            STATE_INITIALIZING,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                let rng = ChaChaRng::new(combined);
                *GLOBAL_RNG.lock() = Some(rng);
                GLOBAL_STATE.store(STATE_INITIALIZED, Ordering::Release);
            }
            Err(STATE_INITIALIZING) => {
                while GLOBAL_STATE.load(Ordering::Acquire) == STATE_INITIALIZING {
                    core::hint::spin_loop();
                }
                if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
                    rng.reseed(combined);
                }
            }
            Err(STATE_INITIALIZED) => {
                if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
                    rng.reseed(combined);
                }
            }
            Err(_) => {
                secure_erase_seeds(&mut combined, local_entropy);
                return Err(RngError::NotInitialized);
            }
        }
    } else {
        if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
            rng.reseed(combined);
        }
    }

    secure_erase_seeds(&mut combined, local_entropy);
    Ok(())
}

#[inline]
pub(crate) fn secure_erase_seeds(combined: &mut [u8; 32], mut local: [u8; 32]) {
    for b in combined.iter_mut() {
        unsafe { ptr::write_volatile(b, 0) };
    }
    for b in local.iter_mut() {
        unsafe { ptr::write_volatile(b, 0) };
    }
    compiler_fence();
    memory_fence();
}

pub fn seed_direct(bootloader_entropy: &[u8; 32]) -> RngResult<()> {
    mark_bootloader_entropy_provided();

    let mut combined = [0u8; 32];
    let mut offset = 0;

    while offset < 32 {
        let tsc = get_tsc_entropy();
        let remaining = 32 - offset;
        let copy_len = core::cmp::min(8, remaining);
        combined[offset..offset + copy_len].copy_from_slice(&tsc.to_le_bytes()[..copy_len]);
        offset += copy_len;
    }

    for i in 0..32 {
        combined[i] ^= bootloader_entropy[i];
    }

    if GLOBAL_STATE.load(Ordering::Acquire) != STATE_INITIALIZED {
        match GLOBAL_STATE.compare_exchange(
            STATE_UNINITIALIZED,
            STATE_INITIALIZING,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                let rng = ChaChaRng::new(combined);
                *GLOBAL_RNG.lock() = Some(rng);
                GLOBAL_STATE.store(STATE_INITIALIZED, Ordering::Release);
            }
            Err(STATE_INITIALIZING) => {
                while GLOBAL_STATE.load(Ordering::Acquire) == STATE_INITIALIZING {
                    core::hint::spin_loop();
                }
                if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
                    rng.reseed(combined);
                }
            }
            Err(STATE_INITIALIZED) => {
                if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
                    rng.reseed(combined);
                }
            }
            Err(_) => {
                for b in &mut combined {
                    unsafe { ptr::write_volatile(b, 0) };
                }
                compiler_fence();
                memory_fence();
                return Err(RngError::NotInitialized);
            }
        }
    } else {
        if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
            rng.reseed(combined);
        }
    }

    for b in &mut combined {
        unsafe { ptr::write_volatile(b, 0) };
    }
    compiler_fence();
    memory_fence();

    Ok(())
}
