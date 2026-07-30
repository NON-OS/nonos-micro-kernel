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

// EFI memory descriptor walks and UEFI framebuffer init are inherently
// arch-specific. Other arches will add match arms when their boot trees
// land with their own per-arch init helpers.
pub(super) fn init_arch_memory_and_framebuffer(handoff: &KernelHandoff) {
    match handoff.arch {
        ArchSpecificHandoff::X86_64 { v1 } => {
            crate::arch::init_boot_memory(v1);
        }
        // The device tree's regions rather than EFI descriptors, and no
        // firmware framebuffer to map on this arch.
        #[cfg(target_arch = "aarch64")]
        ArchSpecificHandoff::Aarch64 { info } => {
            crate::arch::aarch64::boot::init_boot_memory(info);
        }
    }
}
