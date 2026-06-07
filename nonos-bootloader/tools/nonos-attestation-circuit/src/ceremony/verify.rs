// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::error::CeremonyError;
use super::hash::hash_params;
use super::params::CeremonyParams;
use super::record::ContributionRecord;

pub fn verify_contribution(
    prev: &CeremonyParams,
    new: &CeremonyParams,
    rec: &ContributionRecord,
) -> Result<bool, CeremonyError> {
    if new.round != prev.round + 1 {
        return Err(CeremonyError::InvalidRound);
    }
    let prev_hash = hash_params(&prev.pk);
    if prev_hash != rec.previous_params_hash {
        return Err(CeremonyError::HashMismatch);
    }
    let new_hash = hash_params(&new.pk);
    if new_hash != rec.new_params_hash {
        return Err(CeremonyError::HashMismatch);
    }
    if new.pk.vk.gamma_abc_g1.len() != prev.pk.vk.gamma_abc_g1.len() {
        return Err(CeremonyError::InvalidContribution);
    }
    Ok(true)
}
