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
mod page_fault_trampoline;
mod timer_trampoline;
mod wrappers;

pub use exception_trampoline::{br_trampoline, de_trampoline, gpf_trampoline, ud_trampoline};
pub use page_fault_trampoline::page_fault_trampoline;
pub use timer_trampoline::timer_trampoline;
pub use wrappers::{
    irq_keyboard, irq_mouse, irq_syscall, irq_timer, isr_alignment_check, isr_bound_range,
    isr_breakpoint, isr_debug, isr_device_na, isr_divide_error, isr_double_fault, isr_gpf,
    isr_invalid_opcode, isr_invalid_tss, isr_machine_check, isr_nmi, isr_overflow, isr_page_fault,
    isr_segment_not_present, isr_simd_fp, isr_stack_segment_fault, isr_virtualization, isr_x87_fp,
};
