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

// Invariant: the returned `KernelHandoff` borrows `info` for its arch-specific
// tail, so its lifetime is bounded by the boot info the entry path owns.

use super::super::arch::ArchSpecificHandoff;
use super::super::console::EarlyConsole;
use super::super::handoff::KernelHandoff;
use super::builders;
use crate::arch::aarch64::boot::BootInfo;

impl<'a> KernelHandoff<'a> {
    pub fn from_aarch64(info: &'a BootInfo) -> Self {
        Self {
            memory: builders::memory(info),
            cpus: builders::cpus(info),
            console: EarlyConsole::Uart(info.uart_base),
            // QEMU's virt board and the ARM server platforms this targets have
            // no firmware framebuffer. Display comes up later through
            // virtio-gpu, which is a driver, not a handoff.
            framebuffer: None,
            timing: builders::timing(),
            measurement: builders::measurement(),
            arch: ArchSpecificHandoff::Aarch64 { info },
        }
    }
}
