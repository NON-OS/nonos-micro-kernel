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
    encode_response, Request, EINVAL, OP_COUNT, OP_DELETE, OP_LOCK, OP_METADATA, OP_RETRIEVE,
    OP_SIGN_NOX_APPROVE, OP_SIGN_NOX_RECEIPT, OP_STORE, OP_UNLOCK, OP_WALLET_ADDRESS,
    OP_WALLET_GENERATE, OP_WALLET_IMPORT,
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
        OP_WALLET_ADDRESS => handlers::wallet_address(store, req, sender_pid),
        OP_SIGN_NOX_RECEIPT => handlers::sign_receipt(store, req, sender_pid),
        OP_SIGN_NOX_APPROVE => handlers::sign_approve(store, req, sender_pid),
        _ => encode_response(req.seq, EINVAL, &[]),
    }
}
