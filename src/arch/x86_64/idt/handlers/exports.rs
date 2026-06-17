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

pub(crate) use super::isr_exceptions::{
    isr_alignment_check as isr_17, isr_bound_range as isr_5, isr_breakpoint as isr_3,
    isr_control_protection as isr_21, isr_coprocessor_segment as isr_9, isr_debug as isr_1,
    isr_device_not_available as isr_7, isr_divide_error as isr_0, isr_double_fault as isr_8,
    isr_general_protection as isr_13, isr_invalid_opcode as isr_6, isr_invalid_tss as isr_10,
    isr_machine_check as isr_18, isr_nmi as isr_2, isr_overflow as isr_4, isr_page_fault as isr_14,
    isr_reserved_15 as isr_15, isr_reserved_22 as isr_22, isr_reserved_23 as isr_23,
    isr_reserved_24 as isr_24, isr_reserved_25 as isr_25, isr_reserved_26 as isr_26,
    isr_reserved_27 as isr_27, isr_reserved_28 as isr_28, isr_reserved_29 as isr_29,
    isr_reserved_30 as isr_30, isr_reserved_31 as isr_31, isr_segment_not_present as isr_11,
    isr_simd_fp as isr_19, isr_stack_segment as isr_12, isr_virtualization as isr_20,
    isr_x87_fp as isr_16,
};

pub(crate) use super::isr_irqs::{
    isr_irq0 as isr_32, isr_irq1 as isr_33, isr_irq10 as isr_42, isr_irq11 as isr_43,
    isr_irq12 as isr_44, isr_irq13 as isr_45, isr_irq14 as isr_46, isr_irq15 as isr_47,
    isr_irq2 as isr_34, isr_irq3 as isr_35, isr_irq4 as isr_36, isr_irq5 as isr_37,
    isr_irq6 as isr_38, isr_irq7 as isr_39, isr_irq8 as isr_40, isr_irq9 as isr_41, isr_syscall,
};

pub(crate) use super::isr_irqs::{
    isr_ipi_64 as isr_64, isr_ipi_65 as isr_65, isr_ipi_66 as isr_66, isr_ipi_67 as isr_67,
    isr_ipi_68 as isr_68,
};
