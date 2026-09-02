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

# Formatting helpers. Values render into a private scratch line that is
# handed straight to console_write, so callers never manage buffers.

.section .bss
hexline: .skip 80

.text

# print_hex: rdi = value, rsi = digit count (1..16). Low digits of the
# value, most significant first.
.global print_hex
.p2align 4
print_hex:
    lea     r8, [rip + hexline]
    lea     r9, [rip + hexdigits]
    mov     rcx, rsi
1:
    mov     rax, rdi
    lea     rdx, [rcx - 1]
    shl     rdx, 2                      # bit offset of this digit
    push    rcx
    mov     rcx, rdx
    shr     rax, cl
    pop     rcx
    and     eax, 0xF
    mov     al, [r9 + rax]
    mov     [r8], al
    inc     r8
    loop    1b
    lea     rdi, [rip + hexline]
    lea     rsi, [r8]
    sub     rsi, rdi
    jmp     console_write

# print_hex_bytes: rdi = ptr, rsi = byte count (<= 40). Big endian byte
# order, two digits per byte, one console write for the whole run.
.global print_hex_bytes
.p2align 4
print_hex_bytes:
    lea     r8, [rip + hexline]
    lea     r9, [rip + hexdigits]
    mov     rcx, rsi
1:
    movzx   eax, byte ptr [rdi]
    mov     rdx, rax
    shr     eax, 4
    mov     al, [r9 + rax]
    mov     [r8], al
    and     edx, 0xF
    mov     al, [r9 + rdx]
    mov     [r8 + 1], al
    add     r8, 2
    inc     rdi
    loop    1b
    lea     rdi, [rip + hexline]
    lea     rsi, [r8]
    sub     rsi, rdi
    jmp     console_write

.section .rodata
hexdigits: .ascii "0123456789abcdef"
