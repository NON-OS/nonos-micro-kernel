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

macro_rules! irq_entry {
    ($name:ident, $vector:expr) => {
        #[unsafe(naked)]
        #[no_mangle]
        pub(crate) unsafe extern "C" fn $name() {
            core::arch::naked_asm!("push 0", "push {}", "jmp interrupt_common", const $vector);
        }
    };
}

irq_entry!(isr_irq0, 32);
irq_entry!(isr_irq1, 33);
irq_entry!(isr_irq2, 34);
irq_entry!(isr_irq3, 35);
irq_entry!(isr_irq4, 36);
irq_entry!(isr_irq5, 37);
irq_entry!(isr_irq6, 38);
irq_entry!(isr_irq7, 39);
irq_entry!(isr_irq8, 40);
irq_entry!(isr_irq9, 41);
irq_entry!(isr_irq10, 42);
irq_entry!(isr_irq11, 43);
irq_entry!(isr_irq12, 44);
irq_entry!(isr_irq13, 45);
irq_entry!(isr_irq14, 46);
irq_entry!(isr_irq15, 47);
irq_entry!(isr_generic_48, 48);
// Inter-processor interrupt vectors (0x40-0x44). Without these gates the first
// IPI sent once SMP is enabled faults the target CPU; the handlers route
// through the registered other-handlers and signal LAPIC EOI.
irq_entry!(isr_ipi_64, 64);
irq_entry!(isr_ipi_65, 65);
irq_entry!(isr_ipi_66, 66);
irq_entry!(isr_ipi_67, 67);
irq_entry!(isr_ipi_68, 68);
irq_entry!(isr_syscall, 0x80);
