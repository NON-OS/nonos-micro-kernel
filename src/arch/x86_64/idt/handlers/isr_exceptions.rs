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

//! Exception vector gates 0..31, defined in
//! `src/arch/x86_64/asm/vectors.S`. Each pushes a zero where the CPU
//! did not push an error code, then the vector number, and joins the
//! shared save/dispatch body. Declared here so the table builder takes
//! their addresses through the usual exports.

extern "C" {
    pub(crate) fn isr_divide_error();
    pub(crate) fn isr_debug();
    pub(crate) fn isr_nmi();
    pub(crate) fn isr_breakpoint();
    pub(crate) fn isr_overflow();
    pub(crate) fn isr_bound_range();
    pub(crate) fn isr_invalid_opcode();
    pub(crate) fn isr_device_not_available();
    pub(crate) fn isr_double_fault();
    pub(crate) fn isr_coprocessor_segment();
    pub(crate) fn isr_invalid_tss();
    pub(crate) fn isr_segment_not_present();
    pub(crate) fn isr_stack_segment();
    pub(crate) fn isr_general_protection();
    pub(crate) fn isr_page_fault();
    pub(crate) fn isr_reserved_15();
    pub(crate) fn isr_x87_fp();
    pub(crate) fn isr_alignment_check();
    pub(crate) fn isr_machine_check();
    pub(crate) fn isr_simd_fp();
    pub(crate) fn isr_virtualization();
    pub(crate) fn isr_control_protection();
    pub(crate) fn isr_reserved_22();
    pub(crate) fn isr_reserved_23();
    pub(crate) fn isr_reserved_24();
    pub(crate) fn isr_reserved_25();
    pub(crate) fn isr_reserved_26();
    pub(crate) fn isr_reserved_27();
    pub(crate) fn isr_reserved_28();
    pub(crate) fn isr_reserved_29();
    pub(crate) fn isr_reserved_30();
    pub(crate) fn isr_reserved_31();
}
