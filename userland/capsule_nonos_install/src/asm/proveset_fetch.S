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

# The set prover. For every name the compose step read, the four
# artifacts at /install/<name>.elf, .cert, .manifest, .trailer go through
# MkCapsuleVerify, the exact spawn-time chain, without spawning. The
# summary's install caps print with each verdict so the operator sees
# what authority every capsule would carry. One refusal ends the
# ritual; a medium that cannot prove its whole set installs nothing.

.set PREFIX_LEN, 9                      # "/install/"

.section .bss
name_ptr: .skip 8
name_len: .skip 8
len_elf:  .skip 4
len_cert: .skip 4
len_man:  .skip 4
len_trl:  .skip 4

.text

# build_path: rdi extension, rsi extension length. Composes
# "/install/" name "." ext into path_buf; rax is the length or -1 when
# it would not fit.
build_path:
    mov     r8, [rip + name_len]
    lea     rax, [r8 + rsi]
    add     rax, PREFIX_LEN + 1
    cmp     rax, PATH_CAP
    ja      1f
    push    rdi
    push    rsi
    lea     rdi, [rip + path_buf]
    lea     rsi, [rip + set_prefix]
    mov     ecx, PREFIX_LEN
    rep movsb
    mov     rsi, [rip + name_ptr]
    mov     rcx, r8
    rep movsb
    mov     byte ptr [rdi], '.'
    inc     rdi
    pop     rcx
    pop     rsi
    rep movsb
    lea     rax, [rip + path_buf]
    sub     rdi, rax
    mov     rax, rdi
    ret
1:
    mov     rax, -1
    ret

# fetch: rdi ext, rsi ext len, rdx arena, rcx capacity, r8 length slot.
# Reads one artifact whole; the observed length lands in the slot.
fetch:
    push    r8
    push    rdx
    push    rcx
    call    build_path
    test    rax, rax
    js      1f
    lea     rdi, [rip + path_buf]
    mov     rsi, rax
    pop     rcx
    pop     rdx
    call    read_file
    pop     r8
    test    rax, rax
    js      2f
    mov     [r8], eax
    ret
1:
    pop     rcx
    pop     rdx
    pop     r8
    mov     rax, -1
2:
    ret

# prove_one: the current name's four artifacts through MkCapsuleVerify.
# rax 0 with the summary filled, or negative.
