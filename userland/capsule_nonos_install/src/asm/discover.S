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

# Step 1: survey the machine through the device broker. Class 0 asks for
# every device; the broker answers from its claimed-and-probed table, so
# what prints here is the hardware the kernel actually vouches for, not
# a bus scan. DeviceRecord is 176 bytes; the table below holds 16.

.set DEV_STRIDE, 176
.set DEV_MAX,    16

.section .bss
.p2align 3
devtab: .skip DEV_STRIDE * DEV_MAX

.text
.global step_discover
.p2align 4
step_discover:
    push    rbx
    push    r12
    xor     edi, edi                    # class 0: everything
    lea     rsi, [rip + devtab]
    mov     edx, DEV_MAX
    mov     eax, SYS_DEVICE_LIST
    syscall
    test    rax, rax
    js      3f                          # refusal propagates to the ritual
    mov     r12, rax                    # rows written
    lea     rdi, [rip + t_devcount]
    mov     esi, dword ptr [rip + t_devcount_len_w]
    call    console_write
    mov     rdi, r12
    mov     esi, 2
    call    print_hex
    lea     rdi, [rip + nl]
    mov     esi, dword ptr [rip + nl_len_w]
    call    console_write
    xor     ebx, ebx
1:
    cmp     rbx, r12
    jae     2f
    lea     rdi, [rip + t_dev]
    mov     esi, dword ptr [rip + t_dev_len_w]
    call    console_write
    imul    rax, rbx, DEV_STRIDE
    lea     rcx, [rip + devtab]
    mov     rdi, [rcx + rax]            # device_id
    mov     esi, 16
    call    print_hex
    lea     rdi, [rip + t_pci]
    mov     esi, dword ptr [rip + t_pci_len_w]
    call    console_write
    imul    rax, rbx, DEV_STRIDE
    lea     rcx, [rip + devtab]
    movzx   edi, word ptr [rcx + rax + 16]   # vendor
    shl     edi, 16
    movzx   edx, word ptr [rcx + rax + 18]   # device
    or      edi, edx
    mov     esi, 8
    call    print_hex
    lea     rdi, [rip + nl]
    mov     esi, dword ptr [rip + nl_len_w]
    call    console_write
    inc     rbx
    jmp     1b
2:
    xor     eax, eax
3:
    pop     r12
    pop     rbx
    ret
