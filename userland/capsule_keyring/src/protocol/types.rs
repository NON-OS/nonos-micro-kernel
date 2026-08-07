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

pub const OP_STORE: u16 = 1;
pub const OP_RETRIEVE: u16 = 2;
pub const OP_DELETE: u16 = 3;
pub const OP_LOCK: u16 = 4;
pub const OP_UNLOCK: u16 = 5;
pub const OP_METADATA: u16 = 6;
pub const OP_COUNT: u16 = 7;
pub const OP_WALLET_IMPORT: u16 = 8;
pub const OP_WALLET_GENERATE: u16 = 9;
pub const OP_WALLET_ADDRESS: u16 = 10;
pub const OP_SIGN_NOX_RECEIPT: u16 = 11;
pub const OP_SIGN_NOX_APPROVE: u16 = 12;
pub const OP_SIGN_ETH_TRANSFER: u16 = 13;
pub const OP_LIST_WALLET_RAILS: u16 = 14;
pub const OP_WALLET_EXPORT: u16 = 15;
pub const OP_SIGN_NOX_STAKE_APPROVE: u16 = 16;
pub const OP_SIGN_NOX_STAKE: u16 = 17;
pub const OP_SIGN_NOX_TRANSFER: u16 = 18;
pub const OP_SIGN_NOX_UNSTAKE: u16 = 19;
pub const OP_SIGN_NOX_STAKE_LOCKED: u16 = 20;
pub const OP_WALLET_GENERATE_HD: u16 = 19;
pub const OP_WALLET_RECOVER: u16 = 20;

pub const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_0002;

pub(super) const HDR_LEN: usize = 8;

pub struct Request<'a> {
    pub seq: u32,
    pub op: u16,
    pub payload: &'a [u8],
}
