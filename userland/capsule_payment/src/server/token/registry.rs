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

use super::addresses::{ETH_NATIVE, NOX_TOKEN, PRIMER_PENDING};
use super::constants::*;
use super::types::PaymentToken;

pub const TOKENS: &[PaymentToken] = &[
    PaymentToken {
        symbol: b"ETH",
        decimals: 18,
        chain_id: ETHEREUM_MAINNET,
        contract: ETH_NATIVE,
        settlement: SETTLEMENT_NATIVE_ETH,
        flags: TOKEN_ENABLED | TOKEN_NATIVE,
    },
    PaymentToken {
        symbol: b"NOX",
        decimals: 18,
        chain_id: ETHEREUM_MAINNET,
        contract: NOX_TOKEN,
        settlement: SETTLEMENT_NOX_RECEIPT,
        flags: TOKEN_ENABLED | TOKEN_ERC20 | TOKEN_RECEIPT_SETTLED,
    },
    PaymentToken {
        symbol: b"PR",
        decimals: 18,
        chain_id: BASE_MAINNET,
        contract: PRIMER_PENDING,
        settlement: SETTLEMENT_X402_PRIMER,
        flags: TOKEN_ERC20 | TOKEN_X402_SETTLED | TOKEN_CONFIG_REQUIRED,
    },
];
