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

use crate::protocol::{encode_response, Request, EINVAL, ENOSPC};
use crate::store::{eth_secret_valid, KeyType, Store, StoreError};

pub fn wallet_import(store: &mut Store, req: Request<'_>) -> Vec<u8> {
    const HDR: usize = 4 + 8 + 8;
    if req.payload.len() != HDR + 32 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let caller_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let now = u64::from_le_bytes([p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11]]);
    let expires_at = u64::from_le_bytes([p[12], p[13], p[14], p[15], p[16], p[17], p[18], p[19]]);
    let mut secret = [0u8; 32];
    secret.copy_from_slice(&p[20..52]);
    if !eth_secret_valid(&secret) {
        for b in secret.iter_mut() {
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        return encode_response(req.seq, EINVAL, &[]);
    }
    let result = store.store(KeyType::Secp256k1Eth, &secret, caller_pid, now, expires_at);
    for b in secret.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    match result {
        Ok(id) => encode_response(req.seq, 0, &id.to_le_bytes()),
        Err(StoreError::Full) => encode_response(req.seq, ENOSPC, &[]),
        Err(_) => encode_response(req.seq, EINVAL, &[]),
    }
}
