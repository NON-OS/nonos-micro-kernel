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

// Invariant: kernel-core code does not match on this enum. Arch-specific
// callers downcast to their own variant. New arches add a variant when
// their boot tree lands.

use super::super::types::handoff::BootHandoffV1;

#[derive(Debug, Clone, Copy)]
pub enum ArchSpecificHandoff<'a> {
    X86_64 {
        v1: &'a BootHandoffV1,
    },
    /// What the device tree told the aarch64 boot path. Only built on that
    /// arch, because `BootInfo` only exists there.
    #[cfg(target_arch = "aarch64")]
    Aarch64 {
        info: &'a crate::arch::aarch64::boot::BootInfo,
    },
}
