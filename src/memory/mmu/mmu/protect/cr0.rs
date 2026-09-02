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

//! CR0.WP, the bit that decides whether a read-only kernel mapping means
//! anything to ring 0. With it clear, W^X on kernel text is decoration and
//! copy-on-write never faults. The override itself belongs to
//! `arch::paging::write_protect`, which owns the scoped lift used when the
//! kernel deliberately edits a read-only mapping; this only turns it on and
//! reports what the register holds afterwards.

use core::arch::asm;

use super::super::super::constants::CR0_WP;

pub(super) fn enable() -> bool {
    crate::arch::paging::enable_write_protection();
    read() & CR0_WP != 0
}

fn read() -> u64 {
    let cr0: u64;
    // SAFETY: ek@nonos.systems - reading CR0 has no side effect.
    unsafe { asm!("mov {}, cr0", out(reg) cr0, options(nomem, nostack, preserves_flags)) };
    cr0
}
