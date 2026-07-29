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

use super::pool::EntropyPool;
use super::timing::TimingSeed;
use core::sync::atomic::{AtomicU64, Ordering};

static WALLET_COUNTER: AtomicU64 = AtomicU64::new(0xCAFE_BABE_DEAD_BEEF);

pub(super) fn collect(pool: &mut EntropyPool, timing: TimingSeed) {
    let rtc = crate::arch::wall_clock::unix_timestamp().unwrap_or(0);
    let kernel_ms = crate::time::timestamp_millis();

    pool.push_u64(rtc);
    pool.push_u64(kernel_ms);
    pool.push_u64(address_mix(pool));
    pool.push_u64(counter_mix(rtc));
    pool.push_u64(super::super::platform::read_cycle_counter().wrapping_sub(timing.tsc_start));
}

fn address_mix(pool: &EntropyPool) -> u64 {
    let heap_addr = pool.bytes().as_ptr() as u64;
    let stack_addr = super::super::platform::read_stack_pointer();
    heap_addr ^ stack_addr ^ heap_addr.wrapping_mul(0x517c_c1b7_2722_0a95)
}

fn counter_mix(rtc: u64) -> u64 {
    let counter = WALLET_COUNTER.fetch_add(0x9E37_79B9_7F4A_7C15, Ordering::SeqCst);
    let tsc_now = super::super::platform::read_cycle_counter();
    counter ^ tsc_now ^ rtc.wrapping_mul(0xBF58_476D_1CE4_E5B9)
}
