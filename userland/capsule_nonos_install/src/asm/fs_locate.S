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
    mov     esi, dword ptr [rip + fs_name_len_w]
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
