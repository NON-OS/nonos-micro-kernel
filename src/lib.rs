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
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![no_std]
#![cfg_attr(not(feature = "std"), feature(alloc_error_handler))]
#![feature(abi_x86_interrupt)]
#![feature(c_variadic)]
#![feature(thread_local)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unexpected_cfgs)]
#![allow(clippy::integer_division)]
#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

// x86_64 is the production release target. aarch64 and riscv64 are brought up
// behind `nonos-arch-preview`: the arch trees compile and boot in QEMU but are
// not yet release-signed. Without the feature, a non-x86_64 build is refused so
// a release can never ship an unfinished arch by accident.
#[cfg(all(not(target_arch = "x86_64"), not(feature = "nonos-arch-preview")))]
compile_error!(
    "Developer Preview 1.0 ships only x86_64. Build aarch64/riscv64 with \
     --features nonos-arch-preview (QEMU bring-up, not a release target)."
);

#[cfg(all(feature = "nonos-production", feature = "nonos-dev-unverified-capsules"))]
compile_error!(
    "nonos-production and nonos-dev-unverified-capsules are mutually exclusive: \
     production builds must not enable the unverified capsule spawn path."
);

#[cfg(all(feature = "nonos-production", feature = "nonos-zk-rollout"))]
compile_error!(
    "nonos-production and nonos-zk-rollout are mutually exclusive: production \
     builds must enforce attestation, not log-and-continue on a failed proof."
);

#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    entry::handle_oom(layout)
}

pub mod arch;
pub mod boot;
pub mod bus;
pub mod capabilities;
pub mod context;
pub mod crypto;
pub mod drivers;
pub mod elf;
pub mod entry;
pub mod fs;
pub mod hardware;
pub mod interrupts;
pub mod ipc;
pub mod kernel_core;
pub mod log;
pub mod memory;
pub mod process;
pub mod sched;
pub mod security;
pub mod services;
pub mod smp;
pub mod sys;
pub mod syscall;
pub mod time;
pub mod usercopy;
pub mod userspace;

pub use fs as filesystem;
