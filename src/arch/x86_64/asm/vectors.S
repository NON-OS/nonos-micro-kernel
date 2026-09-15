/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

/* Numbered vector gates for the arch-layer IDT.
 *
 * Every gate normalises the stack to vector, error code, iretq frame and
 * falls through to the shared body, which saves the register file plus the
 * data segment, switches ds/es to the kernel selector, and hands the frame
 * to interrupt_dispatch on the Rust side. Vectors whose exception pushes no
 * error code push a zero themselves so the frame layout is uniform.
 */


.intel_syntax noprefix
.text
#include "vec_gate.inc"

/* Exceptions 0 to 31. */
VEC_NOERR isr_divide_error          0
VEC_NOERR isr_debug                 1
VEC_NOERR isr_nmi                   2
VEC_NOERR isr_breakpoint            3
VEC_NOERR isr_overflow              4
VEC_NOERR isr_bound_range           5
VEC_NOERR isr_invalid_opcode        6
VEC_NOERR isr_device_not_available  7
VEC_ERR   isr_double_fault          8
VEC_NOERR isr_coprocessor_segment   9
VEC_ERR   isr_invalid_tss          10
VEC_ERR   isr_segment_not_present  11
VEC_ERR   isr_stack_segment        12
VEC_ERR   isr_general_protection   13
VEC_ERR   isr_page_fault           14
VEC_NOERR isr_reserved_15          15
VEC_NOERR isr_x87_fp               16
VEC_ERR   isr_alignment_check      17
VEC_NOERR isr_machine_check        18
VEC_NOERR isr_simd_fp              19
VEC_NOERR isr_virtualization       20
VEC_ERR   isr_control_protection   21
VEC_NOERR isr_reserved_22          22
VEC_NOERR isr_reserved_23          23
VEC_NOERR isr_reserved_24          24
VEC_NOERR isr_reserved_25          25
VEC_NOERR isr_reserved_26          26
VEC_NOERR isr_reserved_27          27
VEC_NOERR isr_reserved_28          28
VEC_ERR   isr_reserved_29          29
VEC_ERR   isr_reserved_30          30
VEC_NOERR isr_reserved_31          31
