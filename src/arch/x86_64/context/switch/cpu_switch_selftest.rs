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

//! Boot self-test for `cpu_switch`: a kernel↔kernel round-trip (main switches to
//! a target frame; the target switches back). Interrupts off throughout so the
//! legacy preempt path cannot interfere. Gated by `nonos-cpuswitch-selftest`.

use super::cpu_switch::{build_initial_switch_frame, cpu_switch};
use core::sync::atomic::{AtomicU64, Ordering};

static MAIN_RSP: AtomicU64 = AtomicU64::new(0);
static RESULT: AtomicU64 = AtomicU64::new(0);
static mut TMP_STACK: [u64; 512] = [0; 512];

pub fn run() {
    let _irq = crate::interrupts::disable_interrupts_guard();
    let top = unsafe { core::ptr::addr_of!(TMP_STACK) as u64 + 512 * 8 };
    let target_rsp = build_initial_switch_frame(top, target as u64);
    unsafe { cpu_switch(MAIN_RSP.as_ptr(), target_rsp) };
    if RESULT.load(Ordering::SeqCst) == 0x00C0_FFEE {
        crate::sys::serial::println(b"[CPUSWITCH] selftest PASS");
    } else {
        crate::sys::serial::println(b"[CPUSWITCH] selftest FAIL");
    }
}

extern "C" fn target() -> ! {
    RESULT.store(0x00C0_FFEE, Ordering::SeqCst);
    let mut dummy = 0u64;
    unsafe { cpu_switch(&mut dummy as *mut u64, MAIN_RSP.load(Ordering::SeqCst)) };
    loop {
        core::hint::spin_loop();
    }
}
