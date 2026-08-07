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

pub const NOX_TOKEN: [u8; 20] = [
    0x0a, 0x26, 0xc8, 0x0b, 0xe4, 0xe0, 0x60, 0xe6, 0x88, 0xd7, 0xc2, 0x3a, 0xdd, 0xb9, 0x2c, 0xbb,
    0x5d, 0x2c, 0x9e, 0xca,
];

pub const SETTLEMENT: [u8; 20] = [
    0x1b, 0x52, 0x2b, 0x9d, 0x62, 0x98, 0x6f, 0x4a, 0xd0, 0xe7, 0xe8, 0x81, 0xba, 0xd4, 0x64, 0xb6,
    0xe7, 0xe3, 0x73, 0x17,
];

pub const APPROVE_SELECTOR: [u8; 4] = [0x09, 0x5e, 0xa7, 0xb3];

// NOX staking proxy (mainnet), and the stake(uint256) selector. Staking is a
// two-step flow: approve the proxy to move NOX, then call stake on the proxy.
pub const STAKING_PROXY: [u8; 20] = [
    0xa9, 0x4d, 0x60, 0x09, 0x79, 0x0b, 0xa1, 0x35, 0x97, 0xa1, 0xe1, 0xb7, 0xcf, 0x4e, 0x15, 0x31,
    0xea, 0x51, 0x36, 0x13,
];
pub const STAKE_SELECTOR: [u8; 4] = [0xa6, 0x94, 0xfc, 0x3a];

// unstakePosition(uint256) on the staking proxy. The contract closes a whole
// position by its index; there is no call that withdraws an amount.
pub const UNSTAKE_POSITION_SELECTOR: [u8; 4] = [0x4a, 0x23, 0x5e, 0xb6];

// stakeLocked(amount, lockPeriod): the same stake, committed for a term that
// weights it more heavily. The term is seconds, and only the five the
// contract names are accepted.
pub const STAKE_LOCKED_SELECTOR: [u8; 4] = [0x17, 0xb1, 0x8c, 0x89];

// ERC-20 transfer(address,uint256) selector, for sending NOX.
pub const TRANSFER_SELECTOR: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];

pub const CHAIN_ID: u8 = 1;
