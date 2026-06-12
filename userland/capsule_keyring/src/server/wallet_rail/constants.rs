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

pub const FAMILY_ETH_NATIVE: u8 = 1;
pub const FAMILY_ETH_ERC20: u8 = 2;
pub const FAMILY_X402: u8 = 3;
pub const FAMILY_SALVIUM: u8 = 4;

pub const STATUS_ENABLED: u16 = 1;
pub const STATUS_RESERVED: u16 = 2;
pub const STATUS_CONFIG_REQUIRED: u16 = 3;

pub const RAIL_ENABLED: u32 = 1 << 0;
pub const RAIL_NATIVE: u32 = 1 << 1;
pub const RAIL_ERC20: u32 = 1 << 2;
pub const RAIL_KEYRING_SIGNED: u32 = 1 << 3;
pub const RAIL_X402: u32 = 1 << 4;
pub const RAIL_PRIVACY_CHAIN: u32 = 1 << 5;
pub const RAIL_CONFIG_REQUIRED: u32 = 1 << 6;
pub const RAIL_CORE_PORT_REQUIRED: u32 = 1 << 7;

pub const ETHEREUM_MAINNET: u64 = 1;
pub const BASE_MAINNET: u64 = 8453;
pub const SALVIUM_NATIVE: u64 = 0;
