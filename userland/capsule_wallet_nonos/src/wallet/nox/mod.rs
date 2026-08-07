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

//! Live NOX token and staking reads against the pinned mainnet deployment.

mod amount_str;
mod apr_bps;
mod calldata_addr;
pub mod constants;
mod format_apr;
mod format_nox;
mod live_amount;
mod q32_to_u128;
mod stakeable;
mod staking;
mod status;

pub use amount_str::amount_str;
pub use apr_bps::apr_bps;
pub use calldata_addr::calldata_addr;
pub use format_apr::format_apr;
pub use format_nox::format_nox;
pub use live_amount::live_amount;
pub use q32_to_u128::q32_to_u128;
pub use stakeable::{held_wei, NOX_DECIMALS, WEI_PER_NOX};
pub use staking::{
    lock_days, nft_boost_bps, LOCK_TERMS, MAX_POSITIONS, NFT_BOOSTS, SEL_BOOST_MULTIPLIER,
    SEL_GET_POSITION, SEL_GET_STAKE_INFO, SEL_GET_USER_POSITIONS, SEL_TOKEN_OF_OWNER_BY_INDEX,
    SEL_USER_POSITION_SUMMARY, SEL_ZERO_STATE_PASS, SEL_ZSP_BINDING, SEL_ZSP_VALIDLY_BOUND,
};
pub use status::NoxStatus;
