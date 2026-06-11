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
use super::super::function::PSCI_AFFINITY_INFO_64;
use super::super::raw::psci_call2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AffinityState {
    On,
    Off,
    OnPending,
}

pub fn affinity_info(
    target_affinity: u64,
    lowest_affinity_level: u64,
) -> Result<AffinityState, PsciError> {
    let ret = psci_call2(PSCI_AFFINITY_INFO_64, target_affinity, lowest_affinity_level);
    if ret < 0 {
        PsciError::from_ret(ret as i32)?;
    }
    match ret {
        0 => Ok(AffinityState::On),
        1 => Ok(AffinityState::Off),
        2 => Ok(AffinityState::OnPending),
        _ => Err(PsciError::InvalidParams),
    }
}
