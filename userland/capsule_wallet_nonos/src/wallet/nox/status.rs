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

// Live NOX token and staking readout for one wallet, filled from mainnet
// eth_call. Every field carries a ready flag so the UI never renders a value
// that has not actually been read from chain.
#[derive(Clone, Copy)]
pub struct NoxStatus {
    pub balance_ready: bool,
    pub balance_wei: [u8; 32],
    pub claimable_ready: bool,
    pub claimable_wei: [u8; 32],
    pub positions_ready: bool,
    pub positions: u64,
    // ZeroState Passes held. They multiply a stake, so the screen shows them
    // beside the lock term rather than leaving the reader to guess.
    pub passes_ready: bool,
    pub passes: u64,
    pub stats_ready: bool,
    pub total_staked_wei: [u8; 32],
    pub rewards_distributed_wei: [u8; 32],
    pub apr_ready: bool,
    pub apr_bps: u64,
}

impl NoxStatus {
    pub fn empty() -> Self {
        Self {
            balance_ready: false,
            balance_wei: [0; 32],
            claimable_ready: false,
            claimable_wei: [0; 32],
            positions_ready: false,
            positions: 0,
            passes_ready: false,
            passes: 0,
            stats_ready: false,
            total_staked_wei: [0; 32],
            rewards_distributed_wei: [0; 32],
            apr_ready: false,
            apr_bps: 0,
        }
    }
}
