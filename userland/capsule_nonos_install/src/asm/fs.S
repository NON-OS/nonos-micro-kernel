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

# File access over the ramfs service, speaking its wire protocol:
# requests are seq u32, op u16, zero u16, payload; replies are seq u32,
# status i32, payload. Every reply must echo the request's seq or it is
# discarded as malformed; refusals come back as the service's negative
# status, untouched, so the caller sees exactly what the service said.

.set FS_OP_OPEN,   1
.set FS_OP_CLOSE,  2
.set FS_OP_READ,   3
.set FS_RESP_CAP,  4096
.set FS_PATH_MAX,  128
.set FS_TIMEOUT,   5000

.section .data
fs_seq:  .long 1

.section .bss
.p2align 3
fs_port: .skip 4
fs_rlen: .skip 8
fs_req:  .skip FS_PATH_MAX + 32
fs_resp: .skip FS_RESP_CAP + 8

.text

# fs_locate: resolve the ramfs service once. rax is the syscall verdict.
.global fs_locate
.p2align 4
fs_locate:
    lea     rdi, [rip + fs_name]
    mov     esi, fs_name_len
    lea     rdx, [rip + fs_port]
    xor     r10d, r10d
    mov     eax, SYS_SERVICE_LOOKUP
    syscall
    ret

# fs_call: send fs_req of length rdi, reply into fs_resp. Internal.
fs_call:
    mov     rdx, rdi
    mov     edi, dword ptr [rip + fs_port]
    lea     rsi, [rip + fs_req]
    lea     r10, [rip + fs_resp]
    mov     r8d, FS_RESP_CAP + 8
    mov     r9d, FS_TIMEOUT
    mov     eax, SYS_IPC_CALL
    syscall
    ret

# fs_check: rax holds the reply length. Validates the header and echoes
# the service's status back in rax, sign extended; -1 for a malformed or
# mis-sequenced reply. Always advances the sequence number. Internal.
fs_check:
    mov     [rip + fs_rlen], rax
    lea     rcx, [rip + fs_resp]
    cmp     rax, 8
    jl      1f
    mov     edx, dword ptr [rip + fs_seq]
    cmp     dword ptr [rcx], edx
    jne     1f
    movsxd  rax, dword ptr [rcx + 4]
    jmp     2f
1:
    mov     rax, -1
2:
    inc     dword ptr [rip + fs_seq]
    ret

# fs_hdr: stamp seq and op (di) onto fs_req. Internal.
fs_hdr:
    lea     rax, [rip + fs_req]
    mov     ecx, dword ptr [rip + fs_seq]
    mov     [rax], ecx
    mov     [rax + 4], di
    mov     word ptr [rax + 6], 0
    ret

# fs_open: rdi path, rsi length. Returns the handle or a negative status.
.global fs_open
.p2align 4
fs_open:
    push    rbx
    push    r12
    mov     rbx, rdi
    mov     r12, rsi
    cmp     r12, FS_PATH_MAX
    ja      3f
    mov     edi, FS_OP_OPEN
    call    fs_hdr
    lea     rdi, [rip + fs_req]
    mov     dword ptr [rdi + 8], 0      # flags: open existing, nothing else
    mov     [rdi + 12], r12w
    lea     rdi, [rdi + 14]
    mov     rsi, rbx
    mov     rcx, r12
    rep movsb
    lea     rdi, [r12 + 14]
    call    fs_call
    call    fs_check
    test    rax, rax
    jnz     2f
    # A handle needs eight payload bytes; a shorter success reply is a
    # lying peer and gets refused, never read.
    cmp     qword ptr [rip + fs_rlen], 16
    jl      3f
    lea     rcx, [rip + fs_resp]
    mov     rax, [rcx + 8]              # handle
2:
    pop     r12
    pop     rbx
    ret
3:
    mov     rax, -22
    jmp     2b

# fs_read: rdi handle, rsi offset, rdx buffer, rcx capacity. Returns the
# byte count read into the buffer, or a negative status.
.global fs_read
.p2align 4
fs_read:
    push    rbx
    push    r12
    mov     rbx, rdx                    # caller buffer
    mov     r12, rcx                    # capacity
    cmp     r12, FS_RESP_CAP
    jbe     1f
    mov     r12, FS_RESP_CAP
1:
    push    rdi
    push    rsi
    mov     edi, FS_OP_READ
    call    fs_hdr
    lea     rax, [rip + fs_req]
    pop     rsi
    pop     rdi
    mov     [rax + 8], rdi              # handle
    mov     [rax + 16], rsi             # offset
    mov     [rax + 24], r12d            # count
    mov     edi, 28
    call    fs_call
    call    fs_check
    test    rax, rax
    js      2f
    # The count is the peer's word; the copy is bounded by ours.
    mov     rcx, rax
    cmp     rcx, r12
    jbe     4f
    mov     rcx, r12
    mov     rax, r12
4:
    lea     rsi, [rip + fs_resp + 8]
    mov     rdi, rbx
    rep movsb
2:
    pop     r12
    pop     rbx
    ret

# fs_close: rdi handle. The verdict is the service's status.
.global fs_close
.p2align 4
fs_close:
    push    rdi
    mov     edi, FS_OP_CLOSE
    call    fs_hdr
    lea     rax, [rip + fs_req]
    pop     rdi
    mov     [rax + 8], rdi
    mov     edi, 16
    call    fs_call
    call    fs_check
    ret
