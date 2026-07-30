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

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use spin::Mutex;

static ENTROPY_POOL: Mutex<[u8; 512]> = Mutex::new([0u8; 512]);
static ENTROPY_BITS: AtomicU32 = AtomicU32::new(0);
static POOL_INDEX: AtomicU32 = AtomicU32::new(0);
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn init() -> Result<(), &'static str> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    let mut pool = ENTROPY_POOL.lock();
    for chunk in pool.chunks_mut(8) {
        if let Ok(v) = try_secure_random_u64() {
            let bytes = v.to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len().min(8)]);
        }
    }
    ENTROPY_BITS.store(256, Ordering::SeqCst);
    Ok(())
}

pub fn secure_random_u64() -> u64 {
    // Retry the hardware source on each iteration. The previous code sampled
    // once and, on a single transient failure, spun forever WITHOUT retrying,
    // turning a momentary RDRAND/RDSEED hiccup into a permanent hang. Retrying
    // recovers from transient failures; only a sustained, genuinely dead
    // entropy source keeps looping, which is the correct fail-safe: fabricating
    // a value would hand out a predictable key/nonce/canary.
    loop {
        if let Ok(v) = try_secure_random_u64() {
            consume_entropy(64);
            return v;
        }
        core::hint::spin_loop();
    }
}

pub fn try_secure_random_u64() -> Result<u64, &'static str> {
    if let Some(v) = crate::arch::cpu_random::random_u64() {
        return Ok(v);
    }
    if let Some(v) = crate::arch::cpu_random::entropy_u64() {
        return Ok(v);
    }
    if let Ok(v) = try_virtio_rng() {
        return Ok(v);
    }
    Err("No hardware entropy source available")
}

fn try_virtio_rng() -> Result<u64, ()> {
    if crate::drivers::virtio_rng::is_available() {
        let mut buf = [0u8; 8];
        if crate::drivers::virtio_rng::fill_random(&mut buf).is_ok() {
            return Ok(u64::from_le_bytes(buf));
        }
    }
    Err(())
}

pub fn fill_random(buf: &mut [u8]) {
    let mut off = 0;
    while off < buf.len() {
        let v = secure_random_u64();
        let chunk = v.to_le_bytes();
        let remain = buf.len() - off;
        let take = core::cmp::min(remain, chunk.len());
        buf[off..off + take].copy_from_slice(&chunk[..take]);
        off += take;
    }
}

pub fn try_fill_random(buf: &mut [u8]) -> Result<(), &'static str> {
    let mut off = 0;
    while off < buf.len() {
        let v = try_secure_random_u64()?;
        consume_entropy(64);
        let chunk = v.to_le_bytes();
        let take = core::cmp::min(buf.len() - off, chunk.len());
        buf[off..off + take].copy_from_slice(&chunk[..take]);
        off += take;
    }
    Ok(())
}

pub fn secure_random_u32() -> u32 {
    secure_random_u64() as u32
}

pub fn secure_random_u8() -> u8 {
    secure_random_u64() as u8
}

pub fn fill_bytes(buf: &mut [u8]) {
    fill_random(buf)
}

pub fn fill_random_bytes(buf: &mut [u8]) {
    fill_random(buf)
}

pub fn add_entropy(data: &[u8]) {
    if data.is_empty() {
        return;
    }
    let mut pool = ENTROPY_POOL.lock();
    let mut idx = POOL_INDEX.load(Ordering::Relaxed) as usize;
    for &byte in data {
        pool[idx % 512] ^= byte;
        idx = idx.wrapping_add(1);
    }
    POOL_INDEX.store((idx % 512) as u32, Ordering::Relaxed);
    let estimated_bits = (data.len() * 2).min(256) as u32;
    let current = ENTROPY_BITS.load(Ordering::Relaxed);
    ENTROPY_BITS.store((current + estimated_bits).min(4096), Ordering::Relaxed);
}

pub fn get_entropy_count() -> u32 {
    ENTROPY_BITS.load(Ordering::Relaxed)
}

pub fn add_entropy_count(bits: u32) {
    let current = ENTROPY_BITS.load(Ordering::Relaxed);
    ENTROPY_BITS.store((current + bits).min(4096), Ordering::Relaxed);
}

fn consume_entropy(bits: u32) {
    let current = ENTROPY_BITS.load(Ordering::Relaxed);
    ENTROPY_BITS.store(current.saturating_sub(bits), Ordering::Relaxed);
}
