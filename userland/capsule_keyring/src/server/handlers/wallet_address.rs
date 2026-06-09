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

use crate::protocol::{encode_response, Request, EACCES, EINVAL};
use crate::store::{Store, StoreError};

pub fn wallet_address(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    if req.payload.len() != 8 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let payload_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let caller_pid = match super::super::caller::resolve_caller(payload_pid, sender_pid) {
        Some(pid) => pid,
        None => return encode_response(req.seq, EACCES, &[]),
    };
    let id = u32::from_le_bytes([p[4], p[5], p[6], p[7]]);
    let mut secret = match store.eth_secret(id, caller_pid) {
        Ok(s) => s,
        Err(StoreError::AccessDenied) => return encode_response(req.seq, EACCES, &[]),
        Err(_) => return encode_response(req.seq, EINVAL, &[]),
    };
    let mut pubkey = [0u8; 65];
    let rc = nonos_libc::crypto_secp256k1_pubkey(secret.as_ptr(), pubkey.as_mut_ptr());
    for b in secret.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    if rc != 65 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let mut hash = [0u8; 32];
    if nonos_libc::crypto_keccak256(pubkey[1..].as_ptr(), 64, hash.as_mut_ptr(), 32) != 32 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    encode_response(req.seq, 0, &hash[12..32])
}
