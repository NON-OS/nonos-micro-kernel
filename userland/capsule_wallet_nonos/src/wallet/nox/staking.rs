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

//! The staking contract as it is actually deployed (NOXStakingV4 behind the
//! proxy in `constants`). Selectors are keccak of the signatures in the
//! verified ABI; the four read selectors this wallet already used live match
//! the same ABI byte for byte, which is what pins these to the real contract.

/// Positions one account may hold at once.
pub const MAX_POSITIONS: u64 = 10;

/// Read selectors for the position and boost surface.
pub const SEL_GET_USER_POSITIONS: [u8; 4] = [0x2a, 0x6b, 0xc2, 0xdd];
pub const SEL_GET_POSITION: [u8; 4] = [0x3a, 0xdb, 0xb5, 0xaf];
pub const SEL_USER_POSITION_SUMMARY: [u8; 4] = [0xa3, 0xe3, 0x93, 0x1d];
pub const SEL_GET_STAKE_INFO: [u8; 4] = [0xc3, 0x45, 0x31, 0x53];
pub const SEL_ZERO_STATE_PASS: [u8; 4] = [0x42, 0x97, 0x00, 0x1f];
pub const SEL_ZSP_BINDING: [u8; 4] = [0x68, 0x86, 0x40, 0x8a];
pub const SEL_ZSP_VALIDLY_BOUND: [u8; 4] = [0x22, 0x2e, 0x70, 0x4e];
pub const SEL_BOOST_MULTIPLIER: [u8; 4] = [0xf5, 0xbe, 0x47, 0x48];
/// ERC-721 enumeration, for listing the passes an account holds.
pub const SEL_TOKEN_OF_OWNER_BY_INDEX: [u8; 4] = [0x2f, 0x74, 0x5c, 0x59];

/// Lock terms, in seconds, and the weight each earns in basis points. Ten
/// thousand is no boost, so a 365 day lock counts a stake at two and a half
/// times its size.
pub const LOCK_TERMS: [(u32, u32); 6] = [
    (0, 10_000),
    (30 * 86_400, 12_000),
    (60 * 86_400, 14_000),
    (90 * 86_400, 16_000),
    (180 * 86_400, 18_000),
    (365 * 86_400, 25_000),
];

/// Weight earned by holding ZeroState Passes, by how many are held. The
/// contract stops counting at five.
pub const NFT_BOOSTS: [u32; 6] = [10_000, 12_500, 15_000, 17_500, 20_000, 25_000];

/// Weight for a count of passes, saturating at the top of the table.
pub fn nft_boost_bps(count: u64) -> u32 {
    let i = (count as usize).min(NFT_BOOSTS.len() - 1);
    NFT_BOOSTS[i]
}

/// Days a lock term runs, for labelling it.
pub fn lock_days(seconds: u32) -> u32 {
    seconds / 86_400
}
