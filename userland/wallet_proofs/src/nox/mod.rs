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

//! The real NOX read helpers from capsule_wallet_nonos, included verbatim.

#[path = "../../../capsule_wallet_nonos/src/wallet/nox/apr_bps.rs"]
pub mod apr_bps;
#[path = "../../../capsule_wallet_nonos/src/wallet/nox/calldata_addr.rs"]
pub mod calldata_addr;
#[path = "../../../capsule_wallet_nonos/src/wallet/nox/constants.rs"]
pub mod constants;
#[path = "../../../capsule_wallet_nonos/src/wallet/nox/format_apr.rs"]
pub mod format_apr;
#[path = "../../../capsule_wallet_nonos/src/wallet/nox/format_nox.rs"]
pub mod format_nox;
#[path = "../../../capsule_wallet_nonos/src/wallet/nox/q32_to_u128.rs"]
pub mod q32_to_u128;
#[allow(dead_code)]
#[path = "../../../capsule_wallet_nonos/src/wallet/nox/stakeable.rs"]
pub mod stakeable;
