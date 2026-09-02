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

# Step 2: read the install set from the medium. The set is the newline
# separated list of capsule names at /install/set; it is echoed to the
# console so the operator sees exactly what would be composed. A medium
# without a set is not an installer medium and the ritual stops, which
# is the point of a proof carrying installer: it refuses rather than
# guesses.

.set SET_CAP, 2048

.section .bss
set_buf: .skip SET_CAP

.text
.global step_compose
.p2align 4
step_compose:
    push    rbx
    call    fs_locate
    test    rax, rax
    js      5f
    lea     rdi, [rip + set_path]
    mov     esi, set_path_len
    call    fs_open
    test    rax, rax
    js      5f
    mov     rbx, rax                    # handle
    mov     rdi, rbx
    xor     esi, esi
    lea     rdx, [rip + set_buf]
    mov     ecx, SET_CAP
    call    fs_read
    push    rax
    mov     rdi, rbx
    call    fs_close
    pop     rax
    test    rax, rax
    js      5f
    jz      5f                          # an empty set installs nothing
    push    rax
    lea     rdi, [rip + t_set]
    mov     esi, t_set_len
    call    console_write
    pop     rsi
    lea     rdi, [rip + set_buf]
    call    console_write
    lea     rdi, [rip + nl]
    mov     esi, nl_len
    call    console_write
    xor     eax, eax
    pop     rbx
    ret
5:
    lea     rdi, [rip + t_noset]
    mov     esi, t_noset_len
    call    console_write
    mov     eax, 1
    pop     rbx
    ret
