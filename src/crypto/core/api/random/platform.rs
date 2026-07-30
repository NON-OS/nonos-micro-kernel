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

use core::sync::atomic::AtomicU64;

pub(super) static KEYGEN_COUNTER: AtomicU64 = AtomicU64::new(0xB5A1_9E37_C4D2_8F6B);

/* DEV NOTES eK@nonos.systems
   Provides random value with fallback to a counter-mixed PRNG when hardware
   entropy is unavailable. The cycle-counter mixing provides reasonable entropy
   for keygen counters but callers requiring cryptographic randomness should
   validate hardware entropy availability first.
*/
#[inline]
pub(super) fn random64_or_counter() -> u64 {
    secure_random64().unwrap_or_else(|| {
        let ticks = read_cycle_counter();
        let counter = KEYGEN_COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        ticks.wrapping_mul(0x5851f42d4c957f2d) ^ counter
    })
}

pub(super) fn secure_random64() -> Option<u64> {
    if let Some(val) = try_cpu_random64() {
        return Some(val);
    }
    if let Some(val) = try_cpu_entropy64() {
        return Some(val);
    }
    if let Some(val) = try_virtio_rng64() {
        return Some(val);
    }
    None
}

fn try_cpu_random64() -> Option<u64> {
    // A zero draw is rejected here, unlike elsewhere in the tree: this path
    // feeds key material, and a stuck-at-zero generator is the one failure a
    // success flag alone would not catch.
    crate::arch::cpu_random::random_u64().filter(|val| *val != 0)
}

fn try_cpu_entropy64() -> Option<u64> {
    crate::arch::cpu_random::entropy_u64()
}

fn try_virtio_rng64() -> Option<u64> {
    let mut buf = [0u8; 8];
    crate::drivers::virtio_rng::fill_random(&mut buf).ok()?;
    Some(u64::from_le_bytes(buf))
}

#[inline]
pub(super) fn read_cycle_counter() -> u64 {
    crate::arch::read_time_counter()
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub(super) fn get_stack_pointer() -> u64 {
    let rsp: u64;
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) rsp, options(nomem, nostack));
    }
    rsp
}
#[cfg(not(target_arch = "x86_64"))]
#[inline]
pub(super) fn get_stack_pointer() -> u64 {
    0
}

#[cfg(target_arch = "x86_64")]
pub(super) fn read_pit_counter() -> u16 {
    const PIT_CHANNEL0: u16 = 0x40;
    const PIT_COMMAND: u16 = 0x43;
    const LATCH_CHANNEL0: u8 = 0x00;
    unsafe {
        core::arch::asm!("out dx, al", in("dx") PIT_COMMAND, in("al") LATCH_CHANNEL0, options(nostack, preserves_flags, nomem));
        let low: u8;
        core::arch::asm!("in al, dx", out("al") low, in("dx") PIT_CHANNEL0, options(nostack, preserves_flags, nomem));
        let high: u8;
        core::arch::asm!("in al, dx", out("al") high, in("dx") PIT_CHANNEL0, options(nostack, preserves_flags, nomem));
        ((high as u16) << 8) | (low as u16)
    }
}
#[cfg(not(target_arch = "x86_64"))]
pub(super) fn read_pit_counter() -> u16 {
    0
}

pub(super) fn read_rtc_timestamp() -> u64 {
    crate::arch::wall_clock::unix_timestamp().unwrap_or(0)
}
