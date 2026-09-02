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

mod exception_trampoline;
mod fault_trampoline;
mod irq_trampoline;
mod page_fault_trampoline;
mod timer_trampoline;
mod tramp_err;
mod tramp_noerr;
mod wrappers;

pub use exception_trampoline::{
    ac_trampoline, bp_trampoline, br_trampoline, db_trampoline, de_trampoline, gpf_trampoline,
    of_trampoline, ss_trampoline, ud_trampoline, xf_trampoline,
};
pub use fault_trampoline::{
    mf_trampoline, nm_trampoline, np_trampoline, ts_trampoline, ve_trampoline,
};
pub use irq_trampoline::{int80_trampoline, keyboard_trampoline, mouse_trampoline};
pub use page_fault_trampoline::page_fault_trampoline;
pub use timer_trampoline::timer_trampoline;
pub use wrappers::{
    irq_timer, isr_alignment_check, isr_bound_range, isr_breakpoint, isr_debug, isr_divide_error,
    isr_double_fault, isr_invalid_opcode, isr_machine_check, isr_nmi, isr_overflow, isr_page_fault,
    isr_simd_fp,
};
