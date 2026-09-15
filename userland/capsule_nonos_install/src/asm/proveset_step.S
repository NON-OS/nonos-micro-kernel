# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

/* Continues proveset.S: see the note at the top of that file. */

.text

.global step_prove
.p2align 4
step_prove:
    push    rbx
    push    r12
    push    r13
    call    boot_verdict
    test    rax, rax
    jnz     6f
    lea     r12, [rip + set_buf]
    mov     r13d, dword ptr [rip + set_len]
    test    r13, r13
    jz      6f
1:
    test    r13, r13
    jz      7f
    # take one line
    mov     [rip + name_ptr], r12
    xor     rbx, rbx
2:
    cmp     rbx, r13
    jae     3f
    cmp     byte ptr [r12 + rbx], 10
    je      3f
    inc     rbx
    jmp     2b
3:
    mov     [rip + name_len], rbx
    test    rbx, rbx
    jz      5f                          # blank line
    lea     rdi, [rip + t_capsule]
    mov     esi, dword ptr [rip + t_capsule_len_w]
    call    console_write
    mov     rdi, [rip + name_ptr]
    mov     rsi, [rip + name_len]
    call    console_write
    call    prove_one
    test    rax, rax
    jnz     6f
    lea     rdi, [rip + t_proved]
    mov     esi, dword ptr [rip + t_proved_len_w]
    call    console_write
    mov     rdi, [rip + mcvf_out]       # summary caps
    mov     esi, 16
    call    print_hex
    lea     rdi, [rip + nl]
    mov     esi, dword ptr [rip + nl_len_w]
    call    console_write
5:
    # advance past the line and its newline
    lea     rax, [rbx + 1]
    cmp     rax, r13
    ja      4f
    add     r12, rax
    sub     r13, rax
    jmp     1b
4:
    xor     r13, r13
    jmp     1b
7:
    xor     eax, eax
    pop     r13
    pop     r12
    pop     rbx
    ret
6:
    lea     rdi, [rip + t_refusal]
    mov     esi, dword ptr [rip + t_refusal_len_w]
    call    console_write
8:
    mov     eax, 1
    pop     r13
    pop     r12
    pop     rbx
    ret
