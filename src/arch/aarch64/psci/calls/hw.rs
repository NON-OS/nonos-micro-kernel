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
use super::super::function::PSCI_NODE_HW_STATE_64;
use super::super::raw::psci_call2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HwState {
    On,
    Off,
    Standby,
}

pub fn node_hw_state(target_cpu: u64, power_level: u64) -> Result<HwState, PsciError> {
    let ret = psci_call2(PSCI_NODE_HW_STATE_64, target_cpu, power_level);
    if ret < 0 {
        PsciError::from_ret(ret as i32)?;
    }
    match ret {
        0 => Ok(HwState::On),
        1 => Ok(HwState::Off),
        2 => Ok(HwState::Standby),
        _ => Err(PsciError::InvalidParams),
    }
}
