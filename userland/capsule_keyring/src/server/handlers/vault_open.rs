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

//! Take a blob back, and make it a key again.
//!
//! The blob arrives from whoever stored it, which means it arrives from
//! outside. It is not trusted: the tag is verified under a key only this
//! machine can derive before a byte of it reaches the store, and a blob that
//! was altered, or sealed elsewhere, or sealed under a different boot state,
//! opens to nothing. The scalar is then checked as a secp256k1 key like any
//! import, because a valid tag proves origin, not that the bytes are a key.

use alloc::vec::Vec;

use crate::protocol::{encode_response, Request, EACCES, EINVAL, ENOENT, ENOSPC};
use crate::store::{eth_secret_valid, KeyType, Store, StoreError};
use crate::vault::{open_secret, VaultError, BLOB_LEN};

pub fn vault_open(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    const HDR: usize = 4 + 8 + 8;
    if req.payload.len() != HDR + BLOB_LEN {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let payload_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let caller_pid = match super::super::caller::resolve_caller(payload_pid, sender_pid) {
        Some(pid) => pid,
        None => return encode_response(req.seq, EACCES, &[]),
    };
    let now = u64::from_le_bytes([p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11]]);
    let expires_at = u64::from_le_bytes([p[12], p[13], p[14], p[15], p[16], p[17], p[18], p[19]]);
    let mut secret = match open_secret(&p[HDR..]) {
        Ok(v) => v,
        Err(VaultError::NoKey) => return encode_response(req.seq, ENOENT, &[]),
        Err(_) => return encode_response(req.seq, EINVAL, &[]),
    };
    let valid = eth_secret_valid(&secret);
    let result = if valid {
        store.store(KeyType::Secp256k1Eth, &secret, caller_pid, now, expires_at)
    } else {
        Err(StoreError::NotFound)
    };
    for b in secret.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    match result {
        Ok(id) => encode_response(req.seq, 0, &id.to_le_bytes()),
        Err(StoreError::Full) => encode_response(req.seq, ENOSPC, &[]),
        Err(_) => encode_response(req.seq, EINVAL, &[]),
    }
}
