/* SYSCALL trap entry. Installed in LSTAR.
 *   in:  rax=nr  rdi..r9=a1..a6 (a4 in r10, rcx clobbered by CPU)
 *   out: rax=return
 * PerCpuData offsets must track smp::percpu::types::PerCpuData. */

.intel_syntax noprefix
.section .text.syscall_entry_asm, "ax", @progbits
.balign 64
.globl syscall_entry_asm
.type  syscall_entry_asm, @function

.set PCPU_KERN_STACK,   0x20
.set PCPU_USER_STACK,   0x28
.set USER_CS,           0x23
.set USER_DS,           0x1B

syscall_entry_asm:
    swapgs
    mov     gs:PCPU_USER_STACK, rsp
    mov     rsp, gs:PCPU_KERN_STACK

    /* Save state. Seven pushes -> rsp = 8 (mod 16). */
    push    rbp
    push    r11
    push    rcx
    push    r10
    push    r9
    push    r8
    push    rax

    /* User ABI -> SysV: nr,a1..a6. */
    mov     rcx, rdx
    mov     rdx, rsi
    mov     rsi, rdi
    mov     rdi, rax
    mov     r8,  [rsp + 0x18]
    mov     r9,  [rsp + 0x08]
    mov     r11, [rsp + 0x10]

    /* Spill a6 as 7th arg, realigns to 16. */
    push    r11
    call    syscall_handler
    add     rsp, 0x10

    /* SyscallSavedFrame{rax, r8, r9, r10, rcx, r11, rbp} at rsp. */
    push    rax
    mov     rdi, rsp
    sub     rsp, 8
    call    syscall_return_signal_hook
    add     rsp, 8
    pop     rax

    pop     r8
    pop     r9
    pop     r10
    pop     rcx
    pop     r11
    pop     rbp
    push    rax
    movabs  rax, 0xffffffffffe08aff
    and     r11, rax
    or      r11, 0x202
    pop     rax

    /* SYSRETQ requires canonical user RIP. */
    bt      rcx, 47
    jc      .L_iret

    mov     rsp, gs:PCPU_USER_STACK
    swapgs
    sysretq

.L_iret:
    push    USER_DS
    push    qword ptr gs:PCPU_USER_STACK
    push    r11
    push    USER_CS
    push    rcx
    swapgs
    iretq

.size syscall_entry_asm, . - syscall_entry_asm
