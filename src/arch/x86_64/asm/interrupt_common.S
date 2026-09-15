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
.intel_syntax noprefix
.text
#include "gprs.inc"

/* The shared body every numbered gate jumps to, with vector and error code
 * already pushed above the iretq frame, so CS sits at rsp + 24 on entry.
 * A gate taken from CPL 3 swaps to the kernel GS base before anything reads
 * through it, the SSE state of whatever was interrupted is saved around the
 * Rust call, and the stack is 16-aligned at that call as the ABI requires.
 */
.p2align 4
interrupt_common:
    test    byte ptr [rsp + 24], 3
    jz      1f
    swapgs
1:
    PUSH_GPRS
    cld
    mov     ax, ds
    push    rax
    mov     ax, 0x10
    mov     ds, ax
    mov     es, ax
    mov     rbp, rsp
    mov     rdi, rbp
    sub     rsp, 528
    and     rsp, -16
    fxsave  [rsp]
    mov     rbx, rsp
    call    interrupt_dispatch
    fxrstor [rbx]
    mov     rsp, rbp
    pop     rax
    mov     ds, ax
    mov     es, ax
    POP_GPRS
    add     rsp, 16
    test    byte ptr [rsp + 8], 3
    jz      2f
    swapgs
2:
    iretq
