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

use core::arch::asm;

use crate::arch::aarch64::asm::vectors_el1_addr;

/// Whether `VBAR_EL1` still points at the vector table the kernel installed.
///
/// Cheaper than the x86_64 equivalent and stricter. There is no descriptor
/// table to walk: every trap on this architecture enters through the one page
/// `VBAR_EL1` names, so redirecting traps means changing that register, and
/// comparing it against the linked address catches it.
///
/// What this does not catch is an attacker who edits the vector page in place
/// rather than repointing the register. That needs the page's contents
/// measured, not its address, and the mapping made read-only after install.
pub fn vectors_installed() -> bool {
    // SAFETY: reading a system register. No memory operand, no side effect.
    let vbar: u64 = unsafe {
        let value: u64;
        asm!("mrs {0}, vbar_el1", out(reg) value, options(nomem, nostack));
        value
    };

    vbar == vectors_el1_addr()
}
