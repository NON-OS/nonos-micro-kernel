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
//! The list is compiled in rather than fetched. A wallet that asks a server
//! for a token list, or for a logo, tells whoever answers which assets the
//! reader holds, before a single transaction is signed. That is a cleaner
//! way to identify somebody than anything on the chain, and it would happen
//! every time the window is opened. So the mark is drawn from the glyph
//! below and nothing leaves the machine to render this screen.

/// A tradable asset: what to call it, how to divide it, and where it lives.
#[derive(Clone, Copy)]
pub struct Token {
    pub symbol: &'static str,
    pub name: &'static str,
    /// Contract address, or all zero for the chain's own coin.
    pub address: [u8; 20],
    pub decimals: u8,
    /// Colour the mark is drawn in, so the pair is readable at a glance
    /// without an image to fetch.
    pub tint: u32,
    /// One or two letters drawn inside the mark.
    pub mark: &'static str,
}

impl Token {
    /// Whether this is the chain's own coin rather than a contract.
    ///
    /// A native coin needs no approval before a swap, which is the one place
    /// the distinction changes what the screen asks the reader to do.
    pub fn is_native(&self) -> bool {
        self.address == [0u8; 20]
    }
}

const NOX_ADDR: [u8; 20] = [0u8; 20];

/// USDC on mainnet, 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48.
const USDC_ADDR: [u8; 20] = [
    0xA0, 0xb8, 0x69, 0x91, 0xc6, 0x21, 0x8b, 0x36, 0xc1, 0xd1, 0x9D, 0x4a, 0x2e, 0x9E, 0xb0, 0xcE,
    0x36, 0x06, 0xeB, 0x48,
];

/// Wrapped ether, 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2.
const WETH_ADDR: [u8; 20] = [
    0xC0, 0x2a, 0xaA, 0x39, 0xb2, 0x23, 0xFE, 0x8D, 0x0A, 0x0e, 0x5C, 0x4F, 0x27, 0xeA, 0xD9, 0x08,
    0x3C, 0x75, 0x6C, 0xc2,
];

pub const TOKENS: [Token; 3] = [
    Token {
        symbol: "NOX",
        name: "NONOS",
        address: NOX_ADDR,
        decimals: 18,
        tint: 0xFF5EE7D0,
        mark: "Ø",
    },
    Token {
        symbol: "ETH",
        name: "Ether",
        address: WETH_ADDR,
        decimals: 18,
        tint: 0xFF8C8CF0,
        mark: "E",
    },
    Token {
        symbol: "USDC",
        name: "USD Coin",
        address: USDC_ADDR,
        decimals: 6,
        tint: 0xFF3E86E8,
        mark: "$",
    },
];

/// The token at `index`, wrapping so a cycling control cannot fall off the
/// end of the list.
pub fn token(index: u8) -> &'static Token {
    &TOKENS[(index as usize) % TOKENS.len()]
}

/// How many tokens are on the list.
pub fn count() -> u8 {
    TOKENS.len() as u8
}
