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

use crate::crypto::fill_random;
use core::sync::atomic::{AtomicU16, AtomicU64, Ordering};
use nonos_libc::mk_time_millis;

static COVER_BURST: AtomicU16 = AtomicU16::new(1);
static DELAY_JITTER_MS: AtomicU16 = AtomicU16::new(250);
static NEXT_COVER_MS: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy)]
pub struct TimingPolicy {
    pub cover_burst: u16,
    pub delay_jitter_ms: u16,
}

pub fn install(body: &[u8]) -> bool {
    if body.len() != 4 {
        return false;
    }
    let burst = u16::from_le_bytes([body[0], body[1]]).clamp(1, 8);
    let jitter = u16::from_le_bytes([body[2], body[3]]).clamp(10, 10_000);
    COVER_BURST.store(burst, Ordering::Release);
    DELAY_JITTER_MS.store(jitter, Ordering::Release);
    NEXT_COVER_MS.store(0, Ordering::Release);
    true
}

pub fn policy() -> TimingPolicy {
    TimingPolicy {
        cover_burst: COVER_BURST.load(Ordering::Acquire),
        delay_jitter_ms: DELAY_JITTER_MS.load(Ordering::Acquire),
    }
}

pub fn next_cover_ms() -> u64 {
    NEXT_COVER_MS.load(Ordering::Acquire)
}

pub fn cover_due() -> Result<bool, ()> {
    let now = mk_time_millis();
    if now < 0 {
        return Err(());
    }
    let now = now as u64;
    let next = NEXT_COVER_MS.load(Ordering::Acquire);
    if next != 0 && now < next {
        return Ok(false);
    }
    let delay = jitter_ms()? as u64;
    NEXT_COVER_MS.store(now.saturating_add(delay), Ordering::Release);
    Ok(true)
}

fn jitter_ms() -> Result<u16, ()> {
    let max = DELAY_JITTER_MS.load(Ordering::Acquire).max(1);
    let mut raw = [0u8; 2];
    fill_random(&mut raw).map_err(|_| ())?;
    let sample = u16::from_le_bytes(raw);
    Ok(10 + sample % max)
}
