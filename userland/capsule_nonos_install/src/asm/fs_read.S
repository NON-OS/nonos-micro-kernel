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
