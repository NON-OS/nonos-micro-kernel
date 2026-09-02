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

// Shared types across all arch backends.
mod types;

// x86 backend: INTx (IO-APIC) + MSI-X. Existing files retain their
// file-level `#![cfg(target_arch = "x86_64")]` gates.
#[cfg(target_arch = "x86_64")]
mod bind;
#[cfg(target_arch = "x86_64")]
pub mod dispatch;
#[cfg(target_arch = "x86_64")]
mod msix_ops;
#[cfg(target_arch = "x86_64")]
mod poll;
#[cfg(target_arch = "x86_64")]
mod records;
#[cfg(target_arch = "x86_64")]
mod release;
#[cfg(target_arch = "x86_64")]
pub mod reserved;
#[cfg(target_arch = "x86_64")]
mod slots;
#[cfg(target_arch = "x86_64")]
mod validate;
#[cfg(target_arch = "x86_64")]
mod wait;

#[cfg(target_arch = "x86_64")]
pub use bind::bind;
#[cfg(target_arch = "x86_64")]
pub use poll::poll;
#[cfg(target_arch = "x86_64")]
pub use release::{ack_grant, release_all_for_pid, release_for_device, unmap_grant};
#[cfg(target_arch = "x86_64")]
pub use wait::{wait_arm, wait_disarm};

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::{
    ack_grant, bind, poll, release_all_for_pid, release_for_device, unmap_grant, wait_arm,
    wait_disarm,
};

#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "riscv64")]
pub use riscv64::{
    ack_grant, bind, poll, release_all_for_pid, release_for_device, unmap_grant, wait_arm,
    wait_disarm,
};

pub use types::{
    IrqBindError, IrqBindRequest, IrqBindResult, IrqError, IrqGrant, IrqGrantKind, IrqPollResult,
    BIND_MSIX, FLAGS_KNOWN,
};
