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

#[cfg(target_arch = "aarch64")]
mod aarch64;
mod arch;
mod console;
mod cpu;
mod framebuffer;
mod handoff;
mod measurement;
mod memory;
mod timing;
mod x86_64;

pub use arch::ArchSpecificHandoff;
pub use console::EarlyConsole;
pub use cpu::CpuTopology;
pub use framebuffer::Framebuffer;
pub use handoff::KernelHandoff;
pub use measurement::Measurement;
pub use memory::MemoryHandoff;
pub use timing::TimingHandoff;
