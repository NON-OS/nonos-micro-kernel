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

/* Continues proveset.S: see the note at the top of that file. */

.text

prove_one:
    lea     rdi, [rip + e_elf]
    mov     esi, dword ptr [rip + e_elf_len_w]
    lea     rdx, [rip + arena_elf]
    mov     ecx, ELF_CAP
    lea     r8, [rip + len_elf]
    call    fetch
    test    rax, rax
    js      9f
    lea     rdi, [rip + e_cert]
    mov     esi, dword ptr [rip + e_cert_len_w]
    lea     rdx, [rip + arena_cert]
    mov     ecx, CERT_CAP
    lea     r8, [rip + len_cert]
    call    fetch
    test    rax, rax
    js      9f
    lea     rdi, [rip + e_man]
    mov     esi, dword ptr [rip + e_man_len_w]
    lea     rdx, [rip + arena_man]
    mov     ecx, MAN_CAP
    lea     r8, [rip + len_man]
    call    fetch
    test    rax, rax
    js      9f
    lea     rdi, [rip + e_trl]
    mov     esi, dword ptr [rip + e_trl_len_w]
    lea     rdx, [rip + arena_trailer]
    mov     ecx, TRAILER_CAP
    lea     r8, [rip + len_trl]
    call    fetch
    test    rax, rax
    js      9f
    # CapsuleVerifyRequest: four pointers, then four u32 lengths.
    lea     rdi, [rip + mcvf_req]
    lea     rax, [rip + arena_elf]
    mov     [rdi], rax
    lea     rax, [rip + arena_cert]
    mov     [rdi + 8], rax
    lea     rax, [rip + arena_man]
    mov     [rdi + 16], rax
    lea     rax, [rip + arena_trailer]
    mov     [rdi + 24], rax
    mov     eax, dword ptr [rip + len_elf]
    mov     [rdi + 32], eax
    mov     eax, dword ptr [rip + len_cert]
    mov     [rdi + 36], eax
    mov     eax, dword ptr [rip + len_man]
    mov     [rdi + 40], eax
    mov     eax, dword ptr [rip + len_trl]
    mov     [rdi + 44], eax
    lea     rsi, [rip + mcvf_out]
    mov     eax, SYS_CAPSULE_VERIFY
    syscall
9:
    ret

# step_prove: boot verdict first, then the whole set, one refusal ends
# the ritual. The set was read by compose; an untouched set length
# means compose never ran and the step refuses.
