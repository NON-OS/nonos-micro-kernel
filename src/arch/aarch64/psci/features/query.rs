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
use super::super::function::*;
use super::super::raw::psci_call1;

pub fn features(func_id: u32) -> Result<u32, PsciError> {
    let ret = psci_call1(PSCI_FEATURES, func_id as u64);
    if ret < 0 {
        PsciError::from_ret(ret as i32)?;
    }
    Ok(ret as u32)
}

pub fn is_function_supported(func_id: u32) -> bool {
    features(func_id).is_ok()
}

pub fn has_cpu_suspend() -> bool {
    is_function_supported(PSCI_CPU_SUSPEND_64)
}

pub fn has_cpu_off() -> bool {
    is_function_supported(PSCI_CPU_OFF)
}

pub fn has_cpu_on() -> bool {
    is_function_supported(PSCI_CPU_ON_64)
}

pub fn has_affinity_info() -> bool {
    is_function_supported(PSCI_AFFINITY_INFO_64)
}

pub fn has_system_off() -> bool {
    is_function_supported(PSCI_SYSTEM_OFF)
}

pub fn has_system_reset() -> bool {
    is_function_supported(PSCI_SYSTEM_RESET)
}

pub fn has_system_reset2() -> bool {
    is_function_supported(PSCI_SYSTEM_RESET2_64)
}

pub fn has_system_suspend() -> bool {
    is_function_supported(PSCI_SYSTEM_SUSPEND_64)
}

pub fn has_mem_protect() -> bool {
    is_function_supported(PSCI_MEM_PROTECT)
}
