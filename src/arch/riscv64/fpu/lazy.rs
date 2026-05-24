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

use crate::arch::riscv64::interrupts::frame::TrapFrame;

use super::current::slot_mut;
use super::enable::{enable_initial, mark_dirty};
use super::restore::restore;





pub fn try_enable_for_current_task(frame: &mut TrapFrame) -> bool {
    let slot = match slot_mut() {
        Some(s) => s,
        None => return false,
    };



    enable_initial();


    unsafe { restore(&slot.ctx) };



    mark_dirty();



    frame.sstatus = (frame.sstatus
        & !crate::arch::riscv64::cpu::csr::SSTATUS_FS_MASK)
        | crate::arch::riscv64::cpu::csr::SSTATUS_FS_DIRTY;

    slot.enabled = true;
    slot.dirty = true;
    true
}
