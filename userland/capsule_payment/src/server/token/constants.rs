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

pub const SETTLEMENT_NATIVE_ETH: u16 = 1;
pub const SETTLEMENT_NOX_RECEIPT: u16 = 2;
pub const SETTLEMENT_X402_PRIMER: u16 = 3;

pub const TOKEN_ENABLED: u32 = 1 << 0;
pub const TOKEN_NATIVE: u32 = 1 << 1;
pub const TOKEN_ERC20: u32 = 1 << 2;
pub const TOKEN_RECEIPT_SETTLED: u32 = 1 << 3;
pub const TOKEN_X402_SETTLED: u32 = 1 << 4;
pub const TOKEN_CONFIG_REQUIRED: u32 = 1 << 5;

pub const ETHEREUM_MAINNET: u64 = 1;
pub const BASE_MAINNET: u64 = 8453;
