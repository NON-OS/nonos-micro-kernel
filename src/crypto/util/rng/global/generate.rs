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

use super::super::entropy::get_entropy64_secure;
use super::super::error::{RngError, RngResult};
use super::init::ensure_initialized;
use super::state::GLOBAL_RNG;

pub fn get_random_bytes() -> [u8; 32] {
    let mut out = [0u8; 32];

    if ensure_initialized().is_ok() {
        if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
            rng.fill_bytes(&mut out);
            return out;
        }
    }

    fill_with_fallback_secure(&mut out);
    out
}

pub fn get_random_bytes_secure() -> RngResult<[u8; 32]> {
    ensure_initialized()?;

    let mut out = [0u8; 32];
    if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
        rng.fill_bytes(&mut out);
        return Ok(out);
    }

    Err(RngError::NotInitialized)
}

pub fn fill_random_bytes(buf: &mut [u8]) {
    if ensure_initialized().is_ok() {
        if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
            rng.fill_bytes(buf);
            return;
        }
    }

    fill_with_fallback_secure(buf);
}

pub fn fill_random_bytes_secure(buf: &mut [u8]) -> RngResult<()> {
    ensure_initialized()?;

    if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
        rng.fill_bytes(buf);
        return Ok(());
    }

    Err(RngError::NotInitialized)
}

pub fn random_u64() -> u64 {
    if ensure_initialized().is_ok() {
        if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
            return rng.next_u64();
        }
    }

    get_entropy64_secure().unwrap_or_else(|_| {
        static FALLBACK_COUNTER: core::sync::atomic::AtomicU64 =
            core::sync::atomic::AtomicU64::new(0);
        let ticks = crate::arch::read_time_counter();
        ticks ^ FALLBACK_COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed)
    })
}

pub fn random_u64_secure() -> RngResult<u64> {
    ensure_initialized()?;

    if let Some(ref mut rng) = *GLOBAL_RNG.lock() {
        return Ok(rng.next_u64());
    }

    Err(RngError::NotInitialized)
}

pub fn random_u32() -> u32 {
    (random_u64() & 0xFFFF_FFFF) as u32
}

pub fn random_u32_secure() -> RngResult<u32> {
    Ok((random_u64_secure()? & 0xFFFF_FFFF) as u32)
}

pub fn random_range(n: u32) -> u32 {
    if n <= 1 {
        return 0;
    }

    if n.is_power_of_two() {
        return random_u32() & (n - 1);
    }

    let threshold = u32::MAX - (u32::MAX % n);

    loop {
        let r = random_u32();
        if r < threshold {
            return r % n;
        }
    }
}

pub fn random_range_secure(n: u32) -> RngResult<u32> {
    if n <= 1 {
        return Ok(0);
    }

    if n.is_power_of_two() {
        return Ok(random_u32_secure()? & (n - 1));
    }

    let threshold = u32::MAX - (u32::MAX % n);

    loop {
        let r = random_u32_secure()?;
        if r < threshold {
            return Ok(r % n);
        }
    }
}

fn fill_with_fallback_secure(buf: &mut [u8]) {
    static FALLBACK_COUNTER: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
    for chunk in buf.chunks_mut(8) {
        let v = get_entropy64_secure().unwrap_or_else(|_| {
            let ticks = crate::arch::read_time_counter();
            ticks ^ FALLBACK_COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed)
        });
        let bytes = v.to_le_bytes();
        for (i, b) in chunk.iter_mut().enumerate() {
            *b = bytes[i];
        }
    }
}
