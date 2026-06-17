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

use super::super::constants::*;
use super::super::entry::IdtEntry;
use super::super::handlers::*;
use super::super::table::Idt;

pub unsafe fn setup_irqs(idt: &mut Idt) {
    idt.entries[32] = IdtEntry::interrupt_gate(isr_32, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[33] = IdtEntry::interrupt_gate(isr_33, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[34] = IdtEntry::interrupt_gate(isr_34, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[35] = IdtEntry::interrupt_gate(isr_35, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[36] = IdtEntry::interrupt_gate(isr_36, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[37] = IdtEntry::interrupt_gate(isr_37, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[38] = IdtEntry::interrupt_gate(isr_38, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[39] = IdtEntry::interrupt_gate(isr_39, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[40] = IdtEntry::interrupt_gate(isr_40, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[41] = IdtEntry::interrupt_gate(isr_41, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[42] = IdtEntry::interrupt_gate(isr_42, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[43] = IdtEntry::interrupt_gate(isr_43, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[44] = IdtEntry::interrupt_gate(isr_44, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[45] = IdtEntry::interrupt_gate(isr_45, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[46] = IdtEntry::interrupt_gate(isr_46, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[47] = IdtEntry::interrupt_gate(isr_47, KERNEL_CS, 0, DPL_KERNEL);
    // Inter-processor interrupt vectors 0x40-0x44 (decimal 64-68). Installed
    // unconditionally: the handlers only fire once SMP sends IPIs, but having
    // the gates present also stops a stray vector in this range from
    // escalating to a fault.
    idt.entries[64] = IdtEntry::interrupt_gate(isr_64, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[65] = IdtEntry::interrupt_gate(isr_65, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[66] = IdtEntry::interrupt_gate(isr_66, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[67] = IdtEntry::interrupt_gate(isr_67, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[68] = IdtEntry::interrupt_gate(isr_68, KERNEL_CS, 0, DPL_KERNEL);
    idt.entries[0x80] = IdtEntry::trap_gate(isr_syscall, KERNEL_CS, 0, DPL_USER);
}
