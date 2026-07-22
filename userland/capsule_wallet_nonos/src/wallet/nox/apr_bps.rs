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

use super::constants::SECONDS_PER_YEAR;

// Staking APR in basis points from the per-second emission rate and the total
// staked amount, both at the same token precision. APR = annual emission over
// total staked; bps scales that by 10000. None when nothing is staked or the
// arithmetic would overflow u128, so the UI shows a dash instead of a guess.
pub fn apr_bps(emission_rate: u128, total_staked: u128) -> Option<u64> {
    if total_staked == 0 {
        return None;
    }
    let annual = emission_rate.checked_mul(SECONDS_PER_YEAR)?;
    let scaled = annual.checked_mul(10_000)?;
    let bps = scaled / total_staked;
    if bps > u64::MAX as u128 {
        return None;
    }
    Some(bps as u64)
}
