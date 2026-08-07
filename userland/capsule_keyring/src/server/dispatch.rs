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

use alloc::vec::Vec;

use super::handlers;
use crate::protocol::{
    encode_response, Request, EINVAL, OP_COUNT, OP_DELETE, OP_LIST_WALLET_RAILS, OP_LOCK,
    OP_METADATA, OP_RETRIEVE, OP_SIGN_ETH_TRANSFER, OP_SIGN_NOX_APPROVE, OP_SIGN_NOX_RECEIPT,
    OP_SIGN_NOX_STAKE, OP_SIGN_NOX_STAKE_APPROVE, OP_SIGN_NOX_STAKE_LOCKED, OP_SIGN_NOX_TRANSFER,
    OP_SIGN_NOX_UNSTAKE, OP_STORE, OP_UNLOCK, OP_WALLET_ADDRESS, OP_WALLET_EXPORT,
    OP_WALLET_GENERATE, OP_WALLET_GENERATE_HD, OP_WALLET_IMPORT, OP_WALLET_RECOVER,
};
use crate::store::Store;

pub fn dispatch(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    match req.op {
        OP_STORE => handlers::store(store, req, sender_pid),
        OP_RETRIEVE => handlers::retrieve(store, req, sender_pid),
        OP_DELETE => handlers::delete(store, req, sender_pid),
        OP_LOCK => handlers::lock(store, req, sender_pid),
        OP_UNLOCK => handlers::unlock(store, req, sender_pid),
        OP_METADATA => handlers::metadata(store, req, sender_pid),
        OP_COUNT => handlers::count(store, req, sender_pid),
        OP_WALLET_IMPORT => handlers::wallet_import(store, req, sender_pid),
        OP_WALLET_GENERATE => handlers::wallet_generate(store, req, sender_pid),
        OP_WALLET_GENERATE_HD => handlers::wallet_generate_hd(store, req, sender_pid),
        OP_WALLET_RECOVER => handlers::wallet_recover(store, req, sender_pid),
        OP_WALLET_ADDRESS => handlers::wallet_address(store, req, sender_pid),
        OP_WALLET_EXPORT => handlers::wallet_export(store, req, sender_pid),
        OP_SIGN_NOX_RECEIPT => handlers::sign_receipt(store, req, sender_pid),
        OP_SIGN_NOX_APPROVE => handlers::sign_approve(store, req, sender_pid),
        OP_SIGN_NOX_STAKE_APPROVE => handlers::sign_stake_approve(store, req, sender_pid),
        OP_SIGN_NOX_STAKE => handlers::sign_stake(store, req, sender_pid),
        OP_SIGN_NOX_UNSTAKE => handlers::sign_unstake(store, req, sender_pid),
        OP_SIGN_NOX_STAKE_LOCKED => handlers::sign_stake_locked(store, req, sender_pid),
        OP_SIGN_NOX_TRANSFER => handlers::sign_nox_transfer(store, req, sender_pid),
        OP_SIGN_ETH_TRANSFER => handlers::sign_eth_transfer(store, req, sender_pid),
        OP_LIST_WALLET_RAILS => handlers::list_wallet_rails(req),
        _ => encode_response(req.seq, EINVAL, &[]),
    }
}
