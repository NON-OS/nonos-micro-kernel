/* Words the syscall entry code leaves on the kernel stack before it calls the
 * handler. The entry code counts its own saves against this and refuses to
 * assemble on a mismatch; asm/syscall.rs reads the same line for Rust. */
.set SYSCALL_FRAME_WORDS, 17
