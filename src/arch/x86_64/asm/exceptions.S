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

/* Exception and interrupt trampolines, one explicit entry per vector.
 *
 * These were macro-generated Rust naked functions; they are now written
 * once each, here, so the exact instructions guarding every kernel entry
 * from user context are a single file a reviewer reads top to bottom.
 * The two bodies live in assembler macros directly below, and every
 * vector is an explicit one-line instantiation naming the C shim it
 * calls, so nothing about the binding is generated or implicit.
 *
 * The contract, identical to the Rust originals:
 *  - swapgs runs on entry and exit exactly when the interrupted CPL was
 *    user, decided from the frame's CS low bits. The frame sits at
 *    rsp + 8 for vectors without an error code and rsp + 16 with one.
 *  - all general registers are saved, rbp anchors the saved area, and
 *    the shim receives a pointer to the hardware frame in rdi (and the
 *    error code in rsi for the vectors that push one).
 *  - fxsave runs over a 16-aligned 512-byte area so the handler may use
 *    SSE without corrupting the interrupted context; fxrstor restores.
 *  - the err variant drops the error code before iretq.
 *
 * Shims are #[no_mangle] extern "C" functions on the Rust side; each
 * reads the frame and forwards to its handler. The IDT installs the
 * trampoline symbols defined here.
 */

.intel_syntax noprefix
.text

/* Body for vectors that push no error code: frame CS at rsp + 8. */
.macro TRAMP_NOERR name shim
.global \name
.p2align 4
\name:
    test    byte ptr [rsp + 8], 3
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
    lea     rdi, [rbp + 120]
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
    test    byte ptr [rsp + 8], 3
    jz      2f
    swapgs
2:
    iretq
.endm

/* Body for vectors that push an error code: frame CS at rsp + 16, the
 * error code handed to the shim in rsi and dropped before iretq. */
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

/* faults and traps without an error code */
TRAMP_NOERR de_trampoline        nonos_trap_de
TRAMP_NOERR db_trampoline        nonos_trap_db
TRAMP_NOERR bp_trampoline        nonos_trap_bp
TRAMP_NOERR of_trampoline        nonos_trap_of
TRAMP_NOERR br_trampoline        nonos_trap_br
TRAMP_NOERR ud_trampoline        nonos_trap_ud
TRAMP_NOERR nm_trampoline        nonos_trap_nm
TRAMP_NOERR mf_trampoline        nonos_trap_mf
TRAMP_NOERR xf_trampoline        nonos_trap_xf
TRAMP_NOERR ve_trampoline        nonos_trap_ve

/* faults that push an error code */
TRAMP_ERR   ts_trampoline        nonos_trap_ts
TRAMP_ERR   np_trampoline        nonos_trap_np
TRAMP_ERR   ss_trampoline        nonos_trap_ss
TRAMP_ERR   gpf_trampoline       nonos_trap_gpf
TRAMP_ERR   ac_trampoline        nonos_trap_ac

/* interrupts */
TRAMP_NOERR keyboard_trampoline  nonos_trap_keyboard
TRAMP_NOERR mouse_trampoline     nonos_trap_mouse
TRAMP_NOERR int80_trampoline     nonos_trap_int80
