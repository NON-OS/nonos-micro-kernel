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

//! SMEP, SMAP and UMIP, each reported from a read-back rather than from what
//! was asked for: a hypervisor may drop a CR4 write it does not implement.

use core::arch::asm;

use super::super::super::constants::{CR4_SMAP, CR4_SMEP, CR4_UMIP};
use super::cpuid::Supported;

#[derive(Clone, Copy)]
pub(super) struct Cr4Live {
    pub smep: bool,
    pub smap: bool,
    pub umip: bool,
}

pub(super) fn enable(have: Supported) -> Cr4Live {
    let mut cr4 = read();
    if have.smep {
        cr4 |= CR4_SMEP;
    }
    if have.smap {
        cr4 |= CR4_SMAP;
    }
    if have.umip {
        cr4 |= CR4_UMIP;
    }
    write(cr4);

    let live = read();
    let smap = live & CR4_SMAP != 0;
    if smap {
        clear_alignment_check();
    }
    Cr4Live { smep: live & CR4_SMEP != 0, smap, umip: live & CR4_UMIP != 0 }
}

fn read() -> u64 {
    let cr4: u64;
    // SAFETY: ek@nonos.systems - reading CR4 has no side effect.
    unsafe { asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack, preserves_flags)) };
    cr4
}

fn write(cr4: u64) {
    // SAFETY: ek@nonos.systems - every bit set above was reported present by
    // CPUID, so none is reserved on this part, and each one only narrows what
    // ring 0 is permitted to do. The rest of the register is written back as
    // it was read.
    unsafe { asm!("mov cr4, {}", in(reg) cr4, options(nomem, nostack, preserves_flags)) };
}

/// SMAP only faults while EFLAGS.AC is clear, so firmware leaving AC set would
/// hand us a CR4 bit that protects nothing. Valid only once SMAP is live,
/// which is why the caller checks the read-back bit first.
fn clear_alignment_check() {
    // SAFETY: ek@nonos.systems - CR4.SMAP reads back set, so `clac` is
    // defined here; it touches no memory and only clears AC.
    unsafe { asm!("clac", options(nomem, nostack)) };
}
