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

#[cfg(target_arch = "x86_64")]
pub mod apic;
pub mod bench;
pub mod boot_log;
pub mod clock;
pub mod diag;
#[cfg(target_arch = "x86_64")]
pub mod gdt;
#[cfg(target_arch = "x86_64")]
pub mod idt;
pub mod io;
#[cfg(feature = "nonos-bench-micro")]
pub mod microbench;
pub mod policy;
pub mod serial;
pub mod sync;
pub mod timer;

pub use clock::{format_time, format_time_full, get_time, init as clock_init, unix_ms, Time};
#[cfg(target_arch = "x86_64")]
pub use gdt::{enable_iopl, setup as gdt_setup};
#[cfg(target_arch = "x86_64")]
pub use idt::setup as idt_setup;
pub use io::{inb, inl, inw, io_wait, outb, outl, outw};
pub use serial::{init as serial_init, print, print_dec, print_hex, print_str, println};

#[cfg(target_arch = "x86_64")]
pub use apic::init as apic_init;
#[cfg(target_arch = "x86_64")]
pub use apic::is_init as apic_is_init;
#[cfg(target_arch = "x86_64")]
pub use apic::{
    disable_irq, enable_irq, eoi, init_ioapic, init_local_apic, ioapic_set_irq, irq_to_vector,
    setup_keyboard_irq, setup_mouse_irq, setup_timer, stop_timer, IRQ_CASCADE, IRQ_COM1, IRQ_COM2,
    IRQ_COPROCESSOR, IRQ_FLOPPY, IRQ_FREE1, IRQ_FREE2, IRQ_FREE3, IRQ_KEYBOARD, IRQ_LPT1, IRQ_LPT2,
    IRQ_MOUSE, IRQ_PRIMARY_ATA, IRQ_RTC, IRQ_SECONDARY_ATA, IRQ_TIMER, TIMER_VECTOR, VECTOR_COM1,
    VECTOR_KEYBOARD, VECTOR_MOUSE, VECTOR_TIMER,
};

pub use timer::init as timer_init;
pub use timer::is_init as timer_is_init;
pub use timer::{
    delay_ms, delay_us, format_uptime, init_default, ms_to_ticks, process_callbacks, rdtsc,
    register_callback, short_delay, stats, ticks_to_ms, ticks_to_ns, ticks_to_us, tsc_frequency,
    unix_timestamp, unix_timestamp_ms, unregister_callback, uptime_ms, uptime_seconds, uptime_us,
    us_to_ticks, Stopwatch, TimerCallback,
};
