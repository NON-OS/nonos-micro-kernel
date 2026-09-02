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

# Step 3: read the boot chain's own verdict. MkAttestStatus hands back
# the measurements the bootloader recorded before the kernel ran: the
# STARK verification bit, the kernel signature bit, secure boot, and the
# two 32-byte roots. Nothing is recomputed here; an installer that
# cannot show this record refuses to continue.

.section .bss
attbuf: .skip 68                        # AttestStatusAbi: 4 flags + 2 hashes

.text
.global step_prove
.p2align 4
step_prove:
    push    rbx
    lea     rdi, [rip + attbuf]
    mov     eax, SYS_ATTEST_STATUS
    syscall
    test    rax, rax
    js      2f
    lea     rbx, [rip + attbuf]
    xor     ecx, ecx
1:
    push    rcx
    lea     rax, [rip + flagnames]
    mov     rdi, [rax + rcx*8]
    lea     rax, [rip + flaglens]
    mov     rsi, [rax + rcx*8]
    call    console_write
    pop     rcx
    push    rcx
    lea     rdi, [rip + t_yes]
    mov     esi, t_yes_len
    cmp     byte ptr [rbx + rcx], 0
    jne     3f
    lea     rdi, [rip + t_no]
    mov     esi, t_no_len
3:
    call    console_write
    pop     rcx
    inc     rcx
    cmp     rcx, 4
    jb      1b
    lea     rdi, [rip + t_kroot]
    mov     esi, t_kroot_len
    call    console_write
    lea     rdi, [rbx + 4]
    mov     esi, 32
    call    print_hex_bytes
    lea     rdi, [rip + t_proot]
    mov     esi, t_proot_len
    call    console_write
    lea     rdi, [rbx + 36]
    mov     esi, 32
    call    print_hex_bytes
    lea     rdi, [rip + nl]
    mov     esi, nl_len
    call    console_write
    xor     eax, eax
2:
    pop     rbx
    ret

.section .rodata
.p2align 3
flagnames: .quad t_f_zk, t_f_sig, t_f_sb, t_f_att
flaglens:  .quad t_f_zk_len, t_f_sig_len, t_f_sb_len, t_f_att_len
