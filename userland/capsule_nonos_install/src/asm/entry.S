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

.section .rodata
.p2align 3
steps:
    .quad s1, s1_len, step_discover
    .quad s2, s2_len, step_compose
    .quad s3, s3_len, step_prove
    .quad s4, s4_len, step_mint_root
    .quad s5, s5_len, step_enroll
    .quad s6, s6_len, step_write_esp
    .quad s7, s7_len, step_receipt
.set steps_end, .

.text
.global _start
_start:
    lea     rdi, [rip + banner]
    mov     esi, banner_len
    call    console_write
    xor     ebx, ebx                    # step index
1:
    lea     rax, [rip + step_no]
    mov     rcx, rbx
    inc     rcx
    add     cl, '0'
    mov     byte ptr [rax + 1], cl
    mov     rdi, rax
    mov     esi, step_no_len
    call    console_write
    lea     rax, [rip + steps]
    imul    rcx, rbx, 24
    mov     rdi, [rax + rcx]            # name
    mov     rsi, [rax + rcx + 8]        # name length
    call    console_write
    lea     rax, [rip + steps]
    imul    rcx, rbx, 24
    call    qword ptr [rax + rcx + 16]  # handler
    test    rax, rax
    jnz     2f                          # refusal: exit with the step code
    inc     rbx
    cmp     rbx, 7
    jb      1b
    lea     rdi, [rip + done]
    mov     esi, done_len
    call    console_write
    xor     edi, edi
    jmp     3f
2:
    lea     rdi, [rbx + 1]              # exit code is the 1-based failing step
3:
    mov     eax, SYS_EXIT
    syscall
    ud2
