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

// riscv64 PLIC-backed IRQ broker. PLIC external sources only (id
// 1..1023). Timer (sip.STIP) and software (sip.SSIP) interrupts are
// kernel-owned and not delegatable. ACLINT layouts are not supported.
#![cfg(target_arch = "riscv64")]

mod ack;
mod bind;
mod pending;
mod poll;
mod release;
mod trampoline;
mod wait;

pub use ack::ack_grant;
pub use bind::bind;
pub use poll::poll;
pub use release::{release_all_for_pid, release_for_device, unmap_grant};
pub use wait::{wait_arm, wait_disarm};
