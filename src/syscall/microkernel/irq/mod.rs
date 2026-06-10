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

// `MkIrq*` handlers. Cap-gated by `Capability::Irq` at the contract
// layer. Backend lives in `hardware::broker::irq`, selected per arch:
// x86_64 = IO-APIC + MSI-X, aarch64 = GICv3 SPI, riscv64 = PLIC.
// Modes the active arch's broker rejects (e.g. MSI-X on aarch64) come
// back as `UnsupportedFlags` and are mapped to ENOTSUP.

mod ack;
mod bind;
mod errno_map;
mod out;
mod poll;
mod unbind;
mod wait;

pub use ack::sys_irq_ack;
pub use bind::sys_irq_bind;
pub use poll::sys_irq_poll;
pub use unbind::sys_irq_unbind;
pub use wait::sys_irq_wait;
