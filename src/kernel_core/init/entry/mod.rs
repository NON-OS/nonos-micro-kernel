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

mod diagnostics_silenced;
mod fatal;
mod init_arch_firmware;
mod init_arch_framebuffer;
mod init_arch_memory_and_framebuffer;
mod init_core_services;
mod init_runtime;
mod init_vm_and_protection;
mod microkernel_init;
mod microkernel_main;

pub use microkernel_init::microkernel_init;
pub use microkernel_main::microkernel_main;
