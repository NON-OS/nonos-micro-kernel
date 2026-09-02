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

use core::sync::atomic::Ordering;

use super::dispatch::{handle_exception, handle_irq, handle_other, handle_syscall};
use crate::arch::x86_64::idt::constants::IDT_ENTRIES;
use crate::arch::x86_64::idt::entry::InterruptFrame;
use crate::arch::x86_64::idt::state::{
    EXCEPTION_COUNT, INTERRUPT_COUNTS, IRQ_COUNT, TOTAL_INTERRUPTS,
};

// The shared entry body lives in src/arch/x86_64/asm/vectors.S; every
// numbered gate normalises its frame and falls through to it. This is
// the function that body calls with the saved frame.
#[no_mangle]
extern "C" fn interrupt_dispatch(frame: &mut InterruptFrame) {
    let vector = frame.vector as usize;

    if vector < IDT_ENTRIES {
        INTERRUPT_COUNTS[vector].fetch_add(1, Ordering::Relaxed);
    }
    TOTAL_INTERRUPTS.fetch_add(1, Ordering::Relaxed);

    if vector < 32 {
        EXCEPTION_COUNT.fetch_add(1, Ordering::Relaxed);
        handle_exception(frame);
    } else if vector < 48 {
        IRQ_COUNT.fetch_add(1, Ordering::Relaxed);
        handle_irq(frame);
    } else if vector == 0x80 {
        handle_syscall(frame);
    } else {
        handle_other(frame);
    }
}
