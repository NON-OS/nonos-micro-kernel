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

/* Continues fs.S: see the note at the top of that file. */

.text

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
