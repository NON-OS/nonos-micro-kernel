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

mod constants;
mod entry;
mod init;
mod load;
mod table;
mod table_irqs;
mod table_ist;
mod table_trampolines;
pub mod vectors;

pub use constants::{
    DOUBLE_FAULT_IST_INDEX, KEYBOARD_INTERRUPT_ID, MACHINE_CHECK_IST_INDEX, MOUSE_INTERRUPT_ID,
    NMI_IST_INDEX, PAGE_FAULT_IST_INDEX, SYSCALL_INTERRUPT_ID, TIMER_INTERRUPT_ID,
};
pub use entry::{validate_handler_address, validate_ist_index, EntryError, EntryOptions, GateType};
pub use init::init;
pub use load::{
    are_interrupts_enabled, disable_interrupts, enable_interrupts, halt, halt_loop, is_loaded,
    load as load_idt, without_interrupts,
};
pub use table::IDT;
pub use vectors::{
    exception_has_error_code, exception_is_fatal, exception_name, irq_to_vector, is_exception,
    is_irq, is_user_allocatable, vector_to_irq, VECTOR_KEYBOARD, VECTOR_MOUSE, VECTOR_SYSCALL,
    VECTOR_TIMER,
};
