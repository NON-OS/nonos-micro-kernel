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

# Install ritual driver. Prints the banner, then walks the step table:
# announce "[k/7] <name>", run the handler, stop cold on the first
# refusal so a broken step never leaves a half written system behind.
# The step counter is patched into the prefix in place; the digits live
# in writable memory for exactly that reason.

.section .data
step_no: .ascii "[0/7] "
.set step_no_len, . - step_no
.p2align 2
step_no_len_w: .long step_no_len

# The capsule links position independent, so the table stores offsets
# from its own base rather than addresses; the walker adds the base
# back after a rip-relative load. Lengths are plain constants.
.section .rodata
.p2align 3
steps:
    .quad s1 - steps, s1_len, step_discover - steps
    .quad s2 - steps, s2_len, step_compose - steps
    .quad s3 - steps, s3_len, step_prove - steps
    .quad s4 - steps, s4_len, step_mint_root - steps
    .quad s5 - steps, s5_len, step_enroll - steps
    .quad s6 - steps, s6_len, step_write_esp - steps
    .quad s7 - steps, s7_len, step_receipt - steps

.text
.global _start
_start:
    lea     rdi, [rip + banner]
    mov     esi, dword ptr [rip + banner_len_w]
    call    console_write
    xor     ebx, ebx                    # step index
1:
    lea     rax, [rip + step_no]
    mov     rcx, rbx
    inc     rcx
    add     cl, '0'
    mov     byte ptr [rax + 1], cl
    mov     rdi, rax
    mov     esi, dword ptr [rip + step_no_len_w]
    call    console_write
    lea     rax, [rip + steps]
    imul    rcx, rbx, 24
    mov     rdi, [rax + rcx]            # name offset
    add     rdi, rax
    mov     rsi, [rax + rcx + 8]        # name length
    call    console_write
    lea     rax, [rip + steps]
    imul    rcx, rbx, 24
    mov     rdx, [rax + rcx + 16]       # handler offset
    add     rdx, rax
    call    rdx
    test    rax, rax
    jnz     2f                          # refusal: exit with the step code
    inc     rbx
    cmp     rbx, 7
    jb      1b
    lea     rdi, [rip + done]
    mov     esi, dword ptr [rip + done_len_w]
    call    console_write
    xor     edi, edi
    jmp     3f
2:
    lea     rdi, [rbx + 1]              # exit code is the 1-based failing step
3:
    mov     eax, SYS_EXIT
    syscall
    ud2
