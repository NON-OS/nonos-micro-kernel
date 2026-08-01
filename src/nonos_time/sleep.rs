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

use super::{has_mwait_support, now_ns, HighPrecisionTimer};

pub fn sleep_precise_ns(nanoseconds: u64) {
    let timer = HighPrecisionTimer::new();
    while timer.elapsed_ns() < nanoseconds {
        core::hint::spin_loop();
    }
}

pub fn sleep_with_power_mgmt(nanoseconds: u64) {
    let end_time = now_ns() + nanoseconds;
    while now_ns() < end_time {
        // MONITOR/MWAIT parks the core until a wake, and HLT is the fallback on
        // parts without it. Neither has an aarch64 counterpart that is safe to
        // reach for here: `wfe` without an armed event source can wait past the
        // deadline this loop is holding, so the portable path yields and lets the
        // deadline check do the work. Deeper idle on aarch64 wants a timer event
        // set up first, which belongs with the generic timer, not here.
        #[cfg(target_arch = "x86_64")]
        unsafe {
            if has_mwait_support() {
                core::arch::asm!("monitor", in("rax") core::ptr::null::<u8>(), in("rcx") 0u32, in("rdx") 0u32);
                core::arch::asm!("mwait", in("rax") 0u32, in("rcx") 0u32);
            } else {
                x86_64::instructions::hlt();
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        core::hint::spin_loop();
    }
}

pub fn sleep_adaptive(nanoseconds: u64) {
    if nanoseconds < 1000 {
        sleep_precise_ns(nanoseconds);
    } else if nanoseconds < 1_000_000 {
        let spin_time = nanoseconds / 4;
        sleep_precise_ns(spin_time);
        sleep_with_power_mgmt(nanoseconds - spin_time);
    } else {
        sleep_with_power_mgmt(nanoseconds);
    }
}
