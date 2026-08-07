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

//! The tokens this wallet knows how to trade.
//!
//! Compiled in rather than fetched. A wallet that asks a server for a token
//! list, or for a logo, tells whoever answers which assets the reader holds
//! before a single transaction is signed. That identifies somebody more
//! cleanly than most chain analysis, and it would happen every time the
//! window is opened.

use super::kind::Token;

const NOX_ADDR: [u8; 20] = [0u8; 20];

/// USDC, 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48.
const USDC_ADDR: [u8; 20] = [
    0xA0, 0xB8, 0x69, 0x91, 0xC6, 0x21, 0x8B, 0x36, 0xC1, 0xD1, 0x9D, 0x4A, 0x2E, 0x9E, 0xB0, 0xCE,
    0x36, 0x06, 0xEB, 0x48,
];

/// Wrapped ether, 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2.
const WETH_ADDR: [u8; 20] = [
    0xC0, 0x2A, 0xAA, 0x39, 0xB2, 0x23, 0xFE, 0x8D, 0x0A, 0x0E, 0x5C, 0x4F, 0x27, 0xEA, 0xD9, 0x08,
    0x3C, 0x75, 0x6C, 0xC2,
];

pub const TOKENS: [Token; 3] = [
    Token { symbol: "NOX", address: NOX_ADDR, decimals: 18, tint: 0xFF5E_E7D0, mark: "0" },
    Token { symbol: "ETH", address: WETH_ADDR, decimals: 18, tint: 0xFF8C_8CF0, mark: "E" },
    Token { symbol: "USDC", address: USDC_ADDR, decimals: 6, tint: 0xFF3E_86E8, mark: "$" },
];
