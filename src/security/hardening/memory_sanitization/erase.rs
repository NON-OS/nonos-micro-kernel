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

use super::primitives::{memory_fence, volatile_read_u8, volatile_write_u64, volatile_write_u8};
use super::state::{BYTES_SANITIZED, SANITIZATION_CALLS, SANITIZATION_LEVEL};
use super::types::SanitizationLevel;
use core::sync::atomic::Ordering;

// On x86_64 the store loop is the assembly routine in scrub.S, so the wipe
// never depends on what a compiler decides a volatile loop means: rep stosb
// writes the range, sfence orders it, and the audit surface is four
// instructions. Other architectures keep the volatile word walk until they
// grow their own routine.
#[allow(clippy::cast_ptr_alignment)]
#[inline(never)]
pub fn secure_zero(ptr: *mut u8, len: usize) {
    #[cfg(target_arch = "x86_64")]
    // SAFETY: the caller owns len writable bytes at ptr, which is this
    // function's own contract; the routine writes exactly that range.
    unsafe {
        super::scrub_asm::nonos_scrub_bytes(ptr, len)
    };

    #[cfg(not(target_arch = "x86_64"))]
    {
        let align_offset = ptr as usize % 8;
        let start_ptr = ptr;

        for i in 0..core::cmp::min(align_offset, len) {
            // SAFETY: i < len, so ptr.add(i) is within bounds
            volatile_write_u8(unsafe { start_ptr.add(i) }, 0);
        }

        let aligned_start = if align_offset == 0 { 0 } else { 8 - align_offset };
        let aligned_len = (len.saturating_sub(aligned_start)) / 8;

        for i in 0..aligned_len {
            // SAFETY: word_ptr is within the allocated region
            let word_ptr = unsafe { start_ptr.add(aligned_start + i * 8) as *mut u64 };
            volatile_write_u64(word_ptr, 0);
        }

        let suffix_start = aligned_start + aligned_len * 8;
        for i in suffix_start..len {
            // SAFETY: i < len, so ptr.add(i) is within bounds
            volatile_write_u8(unsafe { start_ptr.add(i) }, 0);
        }

        memory_fence();
    }

    BYTES_SANITIZED.fetch_add(len, Ordering::Relaxed);
    SANITIZATION_CALLS.fetch_add(1, Ordering::Relaxed);
}

#[inline(always)]
pub fn secure_zero_slice(slice: &mut [u8]) {
    secure_zero(slice.as_mut_ptr(), slice.len());
}

#[inline(never)]
pub fn dod_5220_erase(ptr: *mut u8, len: usize) {
    for i in 0..len {
        // SAFETY: i < len
        volatile_write_u8(unsafe { ptr.add(i) }, 0x00);
    }
    memory_fence();

    for i in 0..len {
        // SAFETY: i < len
        volatile_write_u8(unsafe { ptr.add(i) }, 0xFF);
    }
    memory_fence();

    let random_seed = crate::crypto::secure_random_u64();
    let mut rng_state = random_seed;
    for i in 0..len {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 7;
        rng_state ^= rng_state << 17;
        // SAFETY: i < len
        volatile_write_u8(unsafe { ptr.add(i) }, rng_state as u8);
    }
    memory_fence();

    for i in 0..len {
        // SAFETY: i < len
        volatile_write_u8(unsafe { ptr.add(i) }, 0x00);
    }
    memory_fence();

    for i in 0..len {
        // SAFETY: i < len
        let value = volatile_read_u8(unsafe { ptr.add(i) });
        if value != 0 {
            crate::log::error!("[SANITIZE] Memory verification failed at offset {}", i);
        }
    }

    BYTES_SANITIZED.fetch_add(len * 4, Ordering::Relaxed);
    SANITIZATION_CALLS.fetch_add(1, Ordering::Relaxed);
}

#[inline(never)]
pub fn paranoid_erase(ptr: *mut u8, len: usize) {
    const PATTERNS: [u8; 7] = [0x00, 0xFF, 0x55, 0xAA, 0x92, 0x49, 0x24];

    for pattern in PATTERNS {
        for i in 0..len {
            // SAFETY: i < len
            volatile_write_u8(unsafe { ptr.add(i) }, pattern);
        }
        memory_fence();
    }

    for i in 0..len {
        // SAFETY: i < len
        volatile_write_u8(unsafe { ptr.add(i) }, 0x00);
    }
    memory_fence();

    BYTES_SANITIZED.fetch_add(len * 8, Ordering::Relaxed);
    SANITIZATION_CALLS.fetch_add(1, Ordering::Relaxed);
}

#[inline(never)]
pub fn gutmann_erase(ptr: *mut u8, len: usize) {
    const GUTMANN_PATTERNS: [u8; 35] = [
        0x55, 0xAA, 0x92, 0x49, 0x24, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
        0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x92, 0x49, 0x24, 0x6D, 0xB6, 0xDB, 0x00, 0xFF, 0x55,
        0xAA, 0x92, 0x49, 0x24, 0x00,
    ];

    for pattern in GUTMANN_PATTERNS {
        for i in 0..len {
            // SAFETY: i < len
            volatile_write_u8(unsafe { ptr.add(i) }, pattern);
        }
        memory_fence();
    }

    let random_seed = crate::crypto::secure_random_u64();
    let mut rng_state = random_seed;
    for i in 0..len {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 7;
        rng_state ^= rng_state << 17;
        // SAFETY: i < len
        volatile_write_u8(unsafe { ptr.add(i) }, rng_state as u8);
    }
    memory_fence();

    for i in 0..len {
        // SAFETY: i < len
        volatile_write_u8(unsafe { ptr.add(i) }, 0x00);
    }
    memory_fence();

    BYTES_SANITIZED.fetch_add(len * 37, Ordering::Relaxed);
    SANITIZATION_CALLS.fetch_add(1, Ordering::Relaxed);
}

pub fn sanitize(ptr: *mut u8, len: usize) {
    let level = SanitizationLevel::from_u64(SANITIZATION_LEVEL.load(Ordering::Relaxed));

    match level {
        SanitizationLevel::None => {}
        SanitizationLevel::Basic => secure_zero(ptr, len),
        SanitizationLevel::Standard => dod_5220_erase(ptr, len),
        SanitizationLevel::Paranoid => paranoid_erase(ptr, len),
        SanitizationLevel::Gutmann => gutmann_erase(ptr, len),
    }
}

pub fn sanitize_slice(slice: &mut [u8]) {
    sanitize(slice.as_mut_ptr(), slice.len());
}
