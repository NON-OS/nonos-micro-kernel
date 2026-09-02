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

pub mod allocation;
#[cfg(target_arch = "x86_64")]
pub mod apic;
#[cfg(target_arch = "x86_64")]
pub mod handlers;
#[cfg(target_arch = "x86_64")]
pub mod idt;
#[cfg(target_arch = "x86_64")]
pub mod isr;
#[cfg(target_arch = "x86_64")]
pub mod pic;
pub mod safety;
pub mod stats;
// The tick counter and the liveness heartbeat. Whatever raises the interrupt,
// counting it and noticing when it stops are the same job everywhere, and this
// module contains no instruction of its own.
pub mod timer;

pub use allocation::{
    allocate_vector, free_vector, get_handler, init as init_interrupt_allocation,
    is_vector_available, register_handler as register_interrupt_handler, unregister_handler,
    ErrorCodeHandler, NoErrorHandler, KEYBOARD_VECTOR, REGISTRY, RESERVED_VECTORS_END,
    SYSCALL_VECTOR, TIMER_VECTOR, VECTOR_COUNT,
};

#[cfg(target_arch = "x86_64")]
pub use apic::{init as init_apic, is_enabled as apic_is_enabled, send_eoi as apic_eoi};

#[cfg(target_arch = "x86_64")]
pub use handlers::{
    alignment_check, bound_range_exceeded, breakpoint, debug, device_not_available, divide_error,
    double_fault, general_protection_fault, invalid_opcode, invalid_tss, keyboard, machine_check,
    mouse, nmi, overflow, page_fault, segment_not_present, simd_floating_point,
    stack_segment_fault, syscall, timer as timer_handler, virtualization, x87_floating_point,
    ExceptionContext, PageFaultContext, PageFaultErrorCode,
};

#[cfg(target_arch = "x86_64")]
pub use idt::{
    exception_has_error_code, exception_is_fatal, exception_name, init as init_idt, irq_to_vector,
    is_exception, is_irq, is_user_allocatable, load_idt, validate_handler_address,
    validate_ist_index, vector_to_irq, EntryError, EntryOptions, GateType, DOUBLE_FAULT_IST_INDEX,
    IDT, KEYBOARD_INTERRUPT_ID, MACHINE_CHECK_IST_INDEX, MOUSE_INTERRUPT_ID, NMI_IST_INDEX,
    PAGE_FAULT_IST_INDEX, SYSCALL_INTERRUPT_ID, TIMER_INTERRUPT_ID,
};

#[cfg(target_arch = "x86_64")]
pub use isr::{
    irq_timer, isr_alignment_check, isr_bound_range, isr_breakpoint, isr_debug, isr_divide_error,
    isr_double_fault, isr_invalid_opcode, isr_machine_check, isr_nmi, isr_overflow, isr_page_fault,
    isr_simd_fp,
};

#[cfg(target_arch = "x86_64")]
pub use pic::{
    get_mask, init as init_pic, mask_all, mask_irq, send_eoi, set_mask, unmask_all, unmask_irq,
    EOI, ICW1_ICW4, ICW1_INIT, ICW4_8086, MASTER_CASCADE_LINE, MASTER_COMMAND, MASTER_DATA,
    MASTER_VECTOR_OFFSET, SLAVE_CASCADE_ID, SLAVE_COMMAND, SLAVE_DATA, SLAVE_VECTOR_OFFSET,
};

pub use stats::{
    get_stats, get_stats_tuple, increment_exceptions, increment_keyboard, increment_mouse,
    increment_page_faults, increment_syscalls, increment_timer, reset_stats, InterruptCounters,
    InterruptStats, COUNTERS,
};

#[cfg(target_arch = "x86_64")]
pub use timer::{
    clear_tick_hook, init as init_timer, on_timer_interrupt, reset_ticks, set_tick_hook, tick,
    tick_count, TickHook, TICK_COUNT,
};

pub use safety::{
    disable_interrupts_guard, in_interrupt_context, set_interrupt_context, InterruptContext,
    InterruptGuard,
};

pub fn get_interrupt_stats() -> InterruptStatsExt {
    let s = get_stats();
    let total = s.timer_ticks
        + s.keyboard_presses
        + s.mouse_events
        + s.syscalls
        + s.page_faults
        + s.exceptions;
    InterruptStatsExt {
        total,
        per_irq: alloc::vec![
            s.timer_ticks,
            s.keyboard_presses,
            s.mouse_events,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0
        ],
    }
}

/// Counts only, read off the tick counter, which every architecture keeps the
/// same way. Nothing here touches a controller, so there is nothing to gate.
pub fn get_softirq_stats() -> SoftirqStats {
    SoftirqStats {
        total: timer::tick_count(),
        per_type: alloc::vec![timer::tick_count(), 0, 0, 0, 0, 0, 0, 0, 0, 0],
    }
}

#[derive(Default, Clone)]
pub struct InterruptStatsExt {
    pub total: u64,
    pub per_irq: alloc::vec::Vec<u64>,
}

#[derive(Default, Clone)]
pub struct SoftirqStats {
    pub total: u64,
    pub per_type: alloc::vec::Vec<u64>,
}
