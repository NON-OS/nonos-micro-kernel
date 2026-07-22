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

pub const KEYRING_SERVICE: &[u8] = b"keyring";
pub const SELF_SERVICE: &[u8] = b"app.nonos_wallet";
pub const OP_WALLET_IMPORT: u16 = 8;
pub const OP_WALLET_GENERATE: u16 = 9;
pub const OP_WALLET_ADDRESS: u16 = 10;
pub const OP_SIGN_NOX_APPROVE: u16 = 12;
pub const OP_SIGN_ETH_TRANSFER: u16 = 13;
pub const OP_LIST_WALLET_RAILS: u16 = 14;
pub const OP_WALLET_EXPORT: u16 = 15;
pub const HDR_LEN: usize = 8;
