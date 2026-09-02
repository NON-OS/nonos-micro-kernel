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

.p2align 4
interrupt_common:
    push    rax
    push    rbx
    push    rcx
    push    rdx
    push    rsi
    push    rdi
    push    rbp
    push    r8
    push    r9
    push    r10
    push    r11
    push    r12
    push    r13
    push    r14
    push    r15
    cld
    mov     ax, ds
    push    rax
    mov     ax, 0x10
    mov     ds, ax
    mov     es, ax
    mov     rdi, rsp
    call    interrupt_dispatch
    pop     rax
    mov     ds, ax
    mov     es, ax
    pop     r15
    pop     r14
    pop     r13
    pop     r12
    pop     r11
    pop     r10
    pop     r9
    pop     r8
    pop     rbp
    pop     rdi
    pop     rsi
    pop     rdx
    pop     rcx
    pop     rbx
    pop     rax
    add     rsp, 16
    iretq

/* CPU pushes no error code: supply the zero, then the vector. */
.macro VEC_NOERR name vector
.global \name
.p2align 4
\name:
    push    0
    push    \vector
    jmp     interrupt_common
.endm

/* CPU pushes the error code: only the vector is missing. */
.macro VEC_ERR name vector
.global \name
.p2align 4
\name:
    push    \vector
    jmp     interrupt_common
.endm

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
VEC_NOERR isr_reserved_29          29
VEC_NOERR isr_reserved_30          30
VEC_NOERR isr_reserved_31          31

VEC_NOERR isr_irq0                 32
VEC_NOERR isr_irq1                 33
VEC_NOERR isr_irq2                 34
VEC_NOERR isr_irq3                 35
VEC_NOERR isr_irq4                 36
VEC_NOERR isr_irq5                 37
VEC_NOERR isr_irq6                 38
VEC_NOERR isr_irq7                 39
VEC_NOERR isr_irq8                 40
VEC_NOERR isr_irq9                 41
VEC_NOERR isr_irq10                42
VEC_NOERR isr_irq11                43
VEC_NOERR isr_irq12                44
VEC_NOERR isr_irq13                45
VEC_NOERR isr_irq14                46
VEC_NOERR isr_irq15                47
VEC_NOERR isr_generic_48           48

/* Inter-processor interrupts. Without these gates the first IPI sent once
 * SMP is enabled faults the target CPU; dispatch routes them through the
 * registered other-handlers and signals LAPIC EOI. */
VEC_NOERR isr_ipi_64               64
VEC_NOERR isr_ipi_65               65
VEC_NOERR isr_ipi_66               66
VEC_NOERR isr_ipi_67               67
VEC_NOERR isr_ipi_68               68

VEC_NOERR isr_syscall             128
