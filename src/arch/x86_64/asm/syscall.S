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
    /* Rust and C string/memory code require DF=0 even if user RFLAGS had DF set. */
    cld
    mov     gs:PCPU_USER_STACK, rsp
    mov     rsp, gs:PCPU_KERN_STACK

    /* Save state. The syscall ABI clobbers only rcx/r11 (CPU) and returns in
       rax; rdi/rsi/rdx must survive so a caller can keep loop state there across
       a syscall (std's byte-write loop holds its offset in rdx). The arg-shuffle
       below destroys them, so stash the user copies under the saved frame and
       restore them on exit. One pad slot keeps 16-alignment since three
       arg-saves are odd. Eleven slots -> rsp = 8 (mod 16). */
    sub     rsp, 8
    push    rdx
    push    rsi
    push    rdi
    push    rbp
    push    r11
    push    rcx
    push    r10
    push    r9
    push    r8
    push    rax

    /* User ABI -> SysV: nr,a1..a6. rdi/rsi/rdx still hold the user args here.
       The frame offsets match the original 7-push layout because the arg-saves
       and pad sit below rbp. */
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

    /* Restore the user's rdi/rsi/rdx and drop the pad. Writing back the user's
       own values honors the ABI (these registers are caller-preserved across a
       syscall) and leaks nothing: no kernel VA or handler leftover reaches ring
       3. rax (return value), rcx (user RIP), and r11 (RFLAGS) are intentional;
       rbx/r12-r15 are callee-saved and still hold user values. */
    pop     rdi
    pop     rsi
    pop     rdx
    add     rsp, 8

    push    rax
    movabs  rax, 0xffffffffffe08aff
    and     r11, rax
    or      r11, 0x202

    /* SYSRETQ takes its #GP for a non-canonical RCX in ring 0, after RSP has
       already been loaded from the user value, so a bad RIP here is an
       escalation primitive and not a fault. Testing bit 47 on its own let
       through anything with bit 47 clear and a higher bit set, which is
       non-canonical; require all of bits 63:47 to be zero, which is also the
       only shape an address in the user half can have. POP does not touch
       flags, so the test survives restoring rax. */
    mov     rax, rcx
    shr     rax, 47
    pop     rax
    jnz     .L_iret

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
