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

//! Which CPU is running this code.

use core::sync::atomic::Ordering;

use super::cpu::apic_to_cpu_id;
use super::state::CPU_COUNT;

/// Resolved from the interrupt controller's id, not from the per-CPU register.
/// A `gs:`-relative read would be one load instead of this scan, but it is only
/// correct while kernel code always runs on the kernel GS base, and this tree
/// does not yet enforce that: NMI, #NM, #MF, #VE and the keyboard, mouse and
/// int-0x80 vectors are reachable from CPL=3 and have no swapgs trampoline, so
/// they run with the user base and a `gs:` read there faults. Move to the
/// per-CPU register once every one of those vectors swapgs-es.
///
/// The answer is never guessed. This used to end in `unwrap_or(0)`, and 0 is
/// the one answer that must never be assumed: the current process is tracked
/// per CPU and every capability check in the syscall layer is keyed on it, so
/// a CPU quietly claiming to be the boot CPU would read and write another
/// CPU's current process and be granted its authority.
#[inline]
pub fn cpu_id() -> usize {
    let apic_id = crate::arch::cpu::get_cpu_id();
    if let Some(index) = apic_to_cpu_id(apic_id) {
        return index;
    }
    if CPU_COUNT.load(Ordering::Acquire) == 0 {
        // Nothing is registered yet, so this is the boot CPU before
        // `smp::init_bsp` filled in its descriptor.
        return 0;
    }
    unregistered(apic_id)
}

/// A CPU the descriptor table does not know about has no per-CPU block, no
/// current-process slot and no time slice. There is no index it can be given
/// that is not a guess, so it stops here instead of running as another CPU.
fn unregistered(apic_id: u32) -> ! {
    crate::sys::serial::print(b"[SMP] FATAL unregistered CPU, APIC id ");
    crate::sys::serial::print_dec(apic_id as u64);
    crate::sys::serial::println(b"");
    crate::arch::halt_loop()
}
