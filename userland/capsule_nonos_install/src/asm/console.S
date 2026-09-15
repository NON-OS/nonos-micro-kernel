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

# Console output for the installer. One routine, one fallback: if the
# console write is refused because no terminal is bound yet during early
# bring up, the same bytes go to the serial debug channel instead, so the
# installer is never silent about what it is doing.

.text

# console_write: rdi = ptr, rsi = len. Clobbers caller saved registers only.
.global console_write
.p2align 4
console_write:
    push    rdi
    push    rsi
    mov     eax, SYS_STDOUT
    syscall
    test    rax, rax
    jns     1f
    pop     rsi
    pop     rdi
    mov     eax, SYS_DEBUG
    syscall
    ret
1:
    add     rsp, 16
    ret
