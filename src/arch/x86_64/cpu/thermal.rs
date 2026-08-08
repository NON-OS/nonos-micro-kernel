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

use super::error::CpuError;
use super::msr::{rdmsr, wrmsr};

const MSR_THERM_STATUS: u32 = 0x19C;
const MSR_TEMPERATURE_TARGET: u32 = 0x1A2;
const MSR_MISC_ENABLE: u32 = 0x1A0;
const MSR_PERF_CTL: u32 = 0x199;
const MSR_PERF_STATUS: u32 = 0x198;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PowerState {
    Performance = 0,
    Balanced = 1,
    PowerSave = 2,
    Minimal = 3,
}

pub fn temperature() -> Result<i32, CpuError> {
    let therm_status = rdmsr(MSR_THERM_STATUS);

    if (therm_status & (1 << 31)) == 0 {
        return Err(CpuError::TemperatureUnavailable);
    }

    let temp_target = rdmsr(MSR_TEMPERATURE_TARGET);
    let tj_max = ((temp_target >> 16) & 0xFF) as i32;
    let digital_readout = ((therm_status >> 16) & 0x7F) as i32;

    Ok(tj_max - digital_readout)
}

pub fn tj_max() -> u8 {
    let temp_target = rdmsr(MSR_TEMPERATURE_TARGET);
    ((temp_target >> 16) & 0xFF) as u8
}

pub fn set_power_state(state: PowerState) -> Result<(), CpuError> {
    let misc_enable = rdmsr(MSR_MISC_ENABLE);

    match state {
        PowerState::Performance => {
            wrmsr(MSR_MISC_ENABLE, misc_enable & !(1 << 38));
            wrmsr(MSR_PERF_CTL, 0);
        }
        PowerState::Balanced => {
            wrmsr(MSR_PERF_CTL, 0x400);
        }
        PowerState::PowerSave => {
            wrmsr(MSR_MISC_ENABLE, misc_enable | (1 << 38));
            wrmsr(MSR_PERF_CTL, 0x800);
        }
        PowerState::Minimal => {
            wrmsr(MSR_PERF_CTL, 0xF00);
        }
    }

    Ok(())
}

pub fn current_pstate() -> u8 {
    let status = rdmsr(MSR_PERF_STATUS);
    ((status >> 8) & 0xFF) as u8
}
