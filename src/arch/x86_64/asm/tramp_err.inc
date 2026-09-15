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
/* Trampoline for a vector that pushes an error code; see exceptions.S. */

.macro TRAMP_ERR name shim
.global \name
.p2align 4
\name:
    test    byte ptr [rsp + 16], 3
    jz      1f
    swapgs
1:
    cld
    push    rax
    push    rcx
    push    rdx
    push    rbx
    push    rbp
    push    rsi
    push    rdi
    push    r8
    push    r9
    push    r10
    push    r11
    push    r12
    push    r13
    push    r14
    push    r15
    mov     rbp, rsp
    lea     rdi, [rbp + 128]
    mov     rsi, [rbp + 120]
    sub     rsp, 528
    and     rsp, -16
    fxsave  [rsp]
    mov     rbx, rsp
    call    \shim
    fxrstor [rbx]
    mov     rsp, rbp
    pop     r15
    pop     r14
    pop     r13
    pop     r12
    pop     r11
    pop     r10
    pop     r9
    pop     r8
    pop     rdi
    pop     rsi
    pop     rbp
    pop     rbx
    pop     rdx
    pop     rcx
    pop     rax
    test    byte ptr [rsp + 16], 3
    jz      2f
    swapgs
2:
    add     rsp, 8
    iretq
.endm
