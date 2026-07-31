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

// The pre-kernel bring-up sequence for a PC: ACPI tables, the local APIC, the
// legacy interrupt controllers, SME/SEV detection and PCI enumeration. It is
// the counterpart of `arch::aarch64::boot::init`, which does the same job with
// the device tree and the GIC, so neither is shared and each stays with its
// own architecture.
#[cfg(target_arch = "x86_64")]
pub mod core_init;
#[cfg(target_arch = "x86_64")]
mod init_memory_encryption;
pub mod mode;

#[cfg(target_arch = "x86_64")]
pub use core_init::init_core_systems;
#[cfg(target_arch = "x86_64")]
pub(super) use init_memory_encryption::init_memory_encryption;
pub use mode::{get_boot_mode, is_microkernel, BootMode};
