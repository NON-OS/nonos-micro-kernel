// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
//! Naked timer-IRQ trampoline.
//!
//! Replaces the `extern "x86-interrupt" irq_timer` wrapper for the
//! timer vector. It saves the full general-purpose register set plus
//! the CPU-pushed iretq frame onto the kernel stack so the trap
//! handler can capture the user context of a CPL=3 capsule that was
//! preempted. Layout produced on the stack (low → high):
//!
//!     [rsp +   0] r15      ← last push
//!     [rsp +   8] r14
//!     [rsp +  16] r13
//!     [rsp +  24] r12
//!     [rsp +  32] r11
//!     [rsp +  40] r10
//!     [rsp +  48] r9
//!     [rsp +  56] r8
//!     [rsp +  64] rdi
//!     [rsp +  72] rsi
//!     [rsp +  80] rbp
//!     [rsp +  88] rbx
//!     [rsp +  96] rdx
//!     [rsp + 104] rcx
//!     [rsp + 112] rax      ← first push
//!     [rsp + 120] rip      ← CPU-pushed
//!     [rsp + 128] cs
//!     [rsp + 136] rflags
//!     [rsp + 144] rsp
//!     [rsp + 152] ss
//!
//! This is exactly the first 160 bytes of `process::userspace::types::
//! UserContext`. The trampoline hands a `*mut UserContext` to the
//! Rust C-ABI handler `timer_trap_handler` which decides whether to
//! capture the frame onto the current PCB and runs the existing
//! timer-tick body. On return from the handler the trampoline pops
//! the GPRs, `swapgs`-es back if returning to CPL=3, and `iretq`s.
//!
//! From CPL=0 the CPU does not switch to TSS.RSP0 — the trampoline
//! runs on whatever kernel stack was already current — and `swapgs`
//! is skipped on both entry and exit.

mod handler;
mod send_eoi;
mod trampoline;

pub use trampoline::timer_trampoline;
