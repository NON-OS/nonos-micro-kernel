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

//! IRQ, IPI, and int 0x80 gates, defined in
//! `src/arch/x86_64/asm/vectors.S`. Same uniform frame as the
//! exception gates: a zero error code, the vector, the shared body.

extern "C" {
    pub(crate) fn isr_irq0();
    pub(crate) fn isr_irq1();
    pub(crate) fn isr_irq2();
    pub(crate) fn isr_irq3();
    pub(crate) fn isr_irq4();
    pub(crate) fn isr_irq5();
    pub(crate) fn isr_irq6();
    pub(crate) fn isr_irq7();
    pub(crate) fn isr_irq8();
    pub(crate) fn isr_irq9();
    pub(crate) fn isr_irq10();
    pub(crate) fn isr_irq11();
    pub(crate) fn isr_irq12();
    pub(crate) fn isr_irq13();
    pub(crate) fn isr_irq14();
    pub(crate) fn isr_irq15();
    pub(crate) fn isr_generic_48();
    pub(crate) fn isr_ipi_64();
    pub(crate) fn isr_ipi_65();
    pub(crate) fn isr_ipi_66();
    pub(crate) fn isr_ipi_67();
    pub(crate) fn isr_ipi_68();
    pub(crate) fn isr_syscall();
}
