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

use super::addresses::{ETH_NATIVE, NOX_TOKEN, PENDING_CONTRACT};
use super::constants::*;
use super::types::WalletRail;

pub const WALLET_RAILS: &[WalletRail] = &[
    WalletRail {
        symbol: b"ETH",
        family: FAMILY_ETH_NATIVE,
        status: STATUS_ENABLED,
        flags: RAIL_ENABLED | RAIL_NATIVE | RAIL_KEYRING_SIGNED,
        chain_id: ETHEREUM_MAINNET,
        contract: ETH_NATIVE,
    },
    WalletRail {
        symbol: b"NOX",
        family: FAMILY_ETH_ERC20,
        status: STATUS_ENABLED,
        flags: RAIL_ENABLED | RAIL_ERC20 | RAIL_KEYRING_SIGNED,
        chain_id: ETHEREUM_MAINNET,
        contract: NOX_TOKEN,
    },
    WalletRail {
        symbol: b"PR",
        family: FAMILY_X402,
        status: STATUS_CONFIG_REQUIRED,
        flags: RAIL_X402 | RAIL_CONFIG_REQUIRED,
        chain_id: BASE_MAINNET,
        contract: PENDING_CONTRACT,
    },
    WalletRail {
        symbol: b"SAL",
        family: FAMILY_SALVIUM,
        status: STATUS_RESERVED,
        flags: RAIL_NATIVE | RAIL_PRIVACY_CHAIN | RAIL_CORE_PORT_REQUIRED,
        chain_id: SALVIUM_NATIVE,
        contract: PENDING_CONTRACT,
    },
];
