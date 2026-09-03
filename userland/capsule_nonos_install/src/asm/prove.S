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

# Step 3: the boot chain's own verdict first, then every capsule named
# by the install set proves itself through MkCapsuleVerify, the same
# chain the kernel runs at spawn. The four artifacts are read whole
# from the medium into a private arena; one refusal stops the ritual.

.set ELF_CAP,     6291456
.set CERT_CAP,    8192
.set MAN_CAP,     8192
.set TRAILER_CAP, 524288
.set PATH_CAP,    128

.section .bss
.p2align 4
arena_elf:     .skip ELF_CAP
arena_cert:    .skip CERT_CAP
arena_man:     .skip MAN_CAP
arena_trailer: .skip TRAILER_CAP
mcvf_req:      .skip 48
mcvf_out:      .skip 144
path_buf:      .skip PATH_CAP

.text

# read_file: rdi path, rsi path length, rdx buffer, rcx capacity.
# Opens, reads to the end in bounded chunks, closes. Returns the total
# or a negative status; a file larger than the buffer is refused, not
# truncated, because a truncated artifact must never reach a verifier.
read_file:
    push    rbx
    push    r12
    push    r13
    push    r14
    mov     r12, rdx                    # buffer base
    mov     r13, rcx                    # capacity
    call    fs_open
    test    rax, rax
    js      8f
    mov     rbx, rax                    # handle
    xor     r14, r14                    # total
1:
    mov     rdi, rbx
    mov     rsi, r14
    lea     rdx, [r12 + r14]
    mov     rcx, r13
    sub     rcx, r14
    jz      7f                          # buffer exactly full: check EOF
    call    fs_read
    test    rax, rax
    js      6f
    jz      2f                          # end of file
    add     r14, rax
    jmp     1b
7:
    # No room left; one more read decides overflow versus exact fit.
    mov     rdi, rbx
    mov     rsi, r14
    lea     rdx, [rip + path_buf]       # scratch, one probe byte
    mov     ecx, 1
    call    fs_read
    test    rax, rax
    jg      5f                          # more data exists: refuse
2:
    mov     rdi, rbx
    call    fs_close
    mov     rax, r14
8:
    pop     r14
    pop     r13
    pop     r12
    pop     rbx
    ret
5:
    mov     rax, -27                    # EFBIG shape: artifact exceeds arena
    jmp     6f
6:
    push    rax
    mov     rdi, rbx
    call    fs_close
    pop     rax
    jmp     8b
