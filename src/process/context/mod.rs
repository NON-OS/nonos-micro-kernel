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

mod cpu_context;
// `full` saves and restores the x86_64 register file from naked functions, which
// are emitted whatever the body says, so the gate has to sit on the module. The
// aarch64 equivalent lives in `arch::aarch64::context` and is reached through the
// facade rather than from here.
#[cfg(target_arch = "x86_64")]
pub mod full;
// Reads and writes a saved `Context`, so it follows `full`.
#[cfg(target_arch = "x86_64")]
mod install;

pub use cpu_context::CpuContext;
// One name, one meaning: the kernel execution point the scheduler saves. Each
// arch saves what its calling convention says has to survive, so the shape
// differs and the API does not.
#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::context::Context;
#[cfg(target_arch = "x86_64")]
pub use full::Context;
#[cfg(target_arch = "x86_64")]
pub use install::{modify_saved_context, read_saved_context};
