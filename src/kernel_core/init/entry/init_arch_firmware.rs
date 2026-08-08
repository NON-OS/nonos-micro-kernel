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

use crate::boot::handoff::{ArchSpecificHandoff, KernelHandoff};

// Firmware tables (ACPI/SMBIOS on x86_64; DTB on aarch64/riscv64) are
// arch-specific. Same shape as the memory/framebuffer downcast.
pub(super) fn init_arch_firmware(handoff: &KernelHandoff) {
    match handoff.arch {
        ArchSpecificHandoff::X86_64 { v1 } => {
            crate::boot::firmware::init(&v1.firmware);
        }
        // The aarch64 firmware table is the DTB, and the boot path walked it
        // into `BootInfo` before kernel-core started: console, GIC, timer and
        // memory all came from there. There is no second pass to run.
        #[cfg(target_arch = "aarch64")]
        ArchSpecificHandoff::Aarch64 { .. } => {}
    }
}
