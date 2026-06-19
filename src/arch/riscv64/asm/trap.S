/* Supervisor trap entry. Direct-mode stvec base.
 * Frame: TrapFrame (288 bytes, 16-aligned).
 * Dispatcher: riscv64_trap_dispatch(*mut TrapFrame).
 * sscratch convention: 0 in S-mode, per-task kernel-sp top in U-mode.
 * user_sp=0 is legal U-mode state, so the first-swap `bnez sp` is the
 * authoritative from-user/from-kernel discriminator. */

.section .text.trap, "ax", %progbits

.equ FRAME_SIZE, 288

.include "trap_save_gprs.inc"
.include "trap_restore_gprs.inc"

.balign 4
.globl __riscv64_trap_entry
.type  __riscv64_trap_entry, @function
__riscv64_trap_entry:
    csrrw   sp, sscratch, sp
    bnez    sp, .Lfrom_user

.Lfrom_kernel:
    csrrw   sp, sscratch, sp           /* undo: sp=kernel, sscratch=0 */
    addi    sp, sp, -FRAME_SIZE
    SAVE_GPRS_NO_SP
    addi    t0, sp, FRAME_SIZE
    sd      t0, 8(sp)
    j       .Lsave_csrs

.Lfrom_user:
    addi    sp, sp, -FRAME_SIZE
    SAVE_GPRS_NO_SP
    csrrw   t0, sscratch, zero         /* t0=user sp; sscratch=0 */
    sd      t0, 8(sp)

.Lsave_csrs:
    csrr    t0, sepc
    sd      t0,  248(sp)
    csrr    t0, sstatus
    sd      t0,  256(sp)
    csrr    t0, scause
    sd      t0,  264(sp)
    csrr    t0, stval
    sd      t0,  272(sp)

    mv      a0, sp
    call    riscv64_trap_dispatch

    ld      t0,  248(sp)
    csrw    sepc, t0
    ld      t0,  256(sp)
    csrw    sstatus, t0

    /* If sret returns to U-mode (SPP=0), prime sscratch with this
     * hart's per-task kernel-sp top so the next user trap can swap. */
    andi    t1, t0, (1 << 8)
    bnez    t1, .Lrestore_gprs
    addi    t1, sp, FRAME_SIZE
    csrw    sscratch, t1

.Lrestore_gprs:
    RESTORE_GPRS_NO_SP
    ld      sp,    8(sp)
    sret

.size __riscv64_trap_entry, . - __riscv64_trap_entry
