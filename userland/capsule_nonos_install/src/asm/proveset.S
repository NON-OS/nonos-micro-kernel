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
.global step_prove
.p2align 4
step_prove:
    push    rbx
    push    r12
    push    r13
    call    boot_verdict
    test    rax, rax
    jnz     6f
    lea     r12, [rip + set_buf]
    mov     r13d, dword ptr [rip + set_len]
    test    r13, r13
    jz      6f
1:
    test    r13, r13
    jz      7f
    # take one line
    mov     [rip + name_ptr], r12
    xor     rbx, rbx
2:
    cmp     rbx, r13
    jae     3f
    cmp     byte ptr [r12 + rbx], 10
    je      3f
    inc     rbx
    jmp     2b
3:
    mov     [rip + name_len], rbx
    test    rbx, rbx
    jz      5f                          # blank line
    lea     rdi, [rip + t_capsule]
    mov     esi, dword ptr [rip + t_capsule_len_w]
    call    console_write
    mov     rdi, [rip + name_ptr]
    mov     rsi, [rip + name_len]
    call    console_write
    call    prove_one
    test    rax, rax
    jnz     6f
    lea     rdi, [rip + t_proved]
    mov     esi, dword ptr [rip + t_proved_len_w]
    call    console_write
    mov     rdi, [rip + mcvf_out]       # summary caps
    mov     esi, 16
    call    print_hex
    lea     rdi, [rip + nl]
    mov     esi, dword ptr [rip + nl_len_w]
    call    console_write
5:
    # advance past the line and its newline
    lea     rax, [rbx + 1]
    cmp     rax, r13
    ja      4f
    add     r12, rax
    sub     r13, rax
    jmp     1b
4:
    xor     r13, r13
    jmp     1b
7:
    xor     eax, eax
    pop     r13
    pop     r12
    pop     rbx
    ret
6:
    lea     rdi, [rip + t_refusal]
    mov     esi, dword ptr [rip + t_refusal_len_w]
    call    console_write
8:
    mov     eax, 1
    pop     r13
    pop     r12
    pop     rbx
    ret
