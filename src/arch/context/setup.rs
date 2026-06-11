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

// Arch-neutral process-spawn handoff. Each per-arch builder validates
// the user VA against its own canonical-user range, looks up the PCB,
// writes a fully-populated `UserEntry` to `pcb.pending_user_entry`.
// kernel_sp is left zero and filled by the scheduler dispatch hook
// from `pcb.kernel_stack_top`.

#[derive(Debug, Clone, Copy)]
pub enum SetupError {
    NoSuchProcess,
    NonUserEntry,
    NonUserStack,
}

pub fn setup_initial_user_pcb(pid: u32, entry: u64, user_sp: u64) -> Result<(), SetupError> {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::context::setup_initial_user_pcb_x86_64(pid, entry, user_sp)
            .map_err(convert_x86)
    }
    #[cfg(target_arch = "aarch64")]
    {
        crate::arch::aarch64::context::setup_initial_user_pcb_aarch64(pid, entry, user_sp)
            .map_err(convert_aarch64)
    }
    #[cfg(target_arch = "riscv64")]
    {
        crate::arch::riscv64::context::setup_initial_user_pcb_riscv64(pid, entry, user_sp)
            .map_err(convert_riscv64)
    }
}

#[cfg(target_arch = "x86_64")]
fn convert_x86(e: crate::arch::x86_64::context::SetupError) -> SetupError {
    use crate::arch::x86_64::context::SetupError as E;
    match e {
        E::NoSuchProcess => SetupError::NoSuchProcess,
        E::NonUserEntry => SetupError::NonUserEntry,
        E::NonUserStack => SetupError::NonUserStack,
    }
}

#[cfg(target_arch = "aarch64")]
fn convert_aarch64(e: crate::arch::aarch64::context::SetupError) -> SetupError {
    use crate::arch::aarch64::context::SetupError as E;
    match e {
        E::NoSuchProcess => SetupError::NoSuchProcess,
        E::NonUserEntry => SetupError::NonUserEntry,
        E::NonUserStack => SetupError::NonUserStack,
    }
}

#[cfg(target_arch = "riscv64")]
fn convert_riscv64(e: crate::arch::riscv64::context::SetupError) -> SetupError {
    use crate::arch::riscv64::context::SetupError as E;
    match e {
        E::NoSuchProcess => SetupError::NoSuchProcess,
        E::NonUserEntry => SetupError::NonUserEntry,
        E::NonUserStack => SetupError::NonUserStack,
    }
}
