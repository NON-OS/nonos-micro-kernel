.code64
nonos_ap_lmode:

    mov     $0x10, %ax
    mov     %ax, %ds
    mov     %ax, %es
    mov     %ax, %fs
    mov     %ax, %gs
    mov     %ax, %ss

    /*
     * Load per-AP stack top, set rsp, align to the ABI, zero rbp.
     *
     * The push is not padding. System V puts a function's entry rsp at 8 mod
     * 16, because a call has just pushed a return address, and the compiler
     * places 16-byte aligned locals against rbp on that basis. The entry below
     * is reached with jmp, which pushes nothing, so a plain 16-aligned rsp
     * hands ap_entry a stack 8 bytes out of contract. rbp then sits at 8 mod
     * 16 in every frame on that cpu, and the first aligned local faults:
     * `movaps %xmm0, -0x30(%rbp)` raises #GP(0). Three application processors
     * died at that one instruction, on three stacks, in the same boot.
     *
     * Nothing shows until the compiler emits an aligned vector access in
     * something an AP runs, so the same kernel brings its APs up or kills them
     * depending on code with no connection to this file.
     *
     * A zero is pushed rather than 8 subtracted so anything walking off the
     * top of the frame chain finds a null return address and stops.
     */
    mov     (TRAMP_BASE + OFF_STACK), %rsp
    and     $-16, %rsp
    pushq   $0
    xor     %rbp, %rbp

    /* SysV first arg: cpu_id. */
    mov     (TRAMP_BASE + OFF_CPU_ID), %edi

    /*
     * No store into the trampoline here: the BSP gates the next AP on the
     * descriptor's Online state (set in the higher-half ap_entry), so the old
     * ready flag was dead. Keeping this stage free of stores is what lets the
     * trampoline page be mapped executable without write, satisfying W^X.
     *
     * Removing the stores this code makes is necessary and was not sufficient.
     * The segment loads above are stores too, made by the processor rather
     * than by an instruction here: loading a selector sets the Accessed bit in
     * its descriptor, and the descriptors live in this page. They are emitted
     * with that bit already set (see tables.S) so the processor has nothing to
     * write. Clear it there and this page needs write permission again.
     */

    /* Jump through stored 64-bit entry pointer. */
    mov     (TRAMP_BASE + OFF_ENTRY), %rax
    jmp     *%rax
