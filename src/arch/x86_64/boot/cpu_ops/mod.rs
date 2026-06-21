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

mod control_regs;
mod control_regs_cr0;
mod control_regs_cr4;
mod cpuid;
mod flags;
mod intrinsics;
mod intrinsics_fence;
mod intrinsics_interrupt;
mod msr;
mod tsc;
mod xcr;

pub use control_regs::{
    read_cr0, read_cr2, read_cr3, read_cr4, read_cr8, write_cr0, write_cr3, write_cr4, write_cr8,
};
pub use cpuid::{cpuid, cpuid_count};
pub use flags::{read_rflags, write_rflags};
pub use intrinsics::{cli, halt_loop, hlt, invlpg, lfence, mfence, pause, sfence, sti};
pub use msr::{rdmsr, wrmsr};
pub use tsc::{rdtsc, rdtscp};
pub use xcr::{read_xcr0, write_xcr0};
