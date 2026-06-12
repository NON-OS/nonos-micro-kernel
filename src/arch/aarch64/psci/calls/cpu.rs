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

use super::super::error::PsciError;
use super::super::function::{
    PSCI_CPU_DEFAULT_SUSPEND_64, PSCI_CPU_OFF, PSCI_CPU_ON_64, PSCI_CPU_SUSPEND_64,
};
use super::super::raw::{psci_call, psci_call0, psci_call2};

pub fn cpu_on(target_cpu: u64, entry_point: u64, context_id: u64) -> Result<(), PsciError> {
    PsciError::from_ret(psci_call(PSCI_CPU_ON_64, target_cpu, entry_point, context_id) as i32)
}

pub fn cpu_off() -> Result<(), PsciError> {
    PsciError::from_ret(psci_call0(PSCI_CPU_OFF) as i32)
}

pub fn cpu_suspend(power_state: u64, entry_point: u64, context_id: u64) -> Result<(), PsciError> {
    PsciError::from_ret(psci_call(PSCI_CPU_SUSPEND_64, power_state, entry_point, context_id) as i32)
}

pub fn cpu_default_suspend(entry_point: u64, context_id: u64) -> Result<(), PsciError> {
    PsciError::from_ret(psci_call2(PSCI_CPU_DEFAULT_SUSPEND_64, entry_point, context_id) as i32)
}
