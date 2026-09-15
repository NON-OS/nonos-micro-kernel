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

//! Hand back a wallet key sealed to this machine, for the caller to store.
//!
//! The keyring holds no filesystem capability, and should not: a service that
//! holds private keys has no business writing files. So it seals and the
//! caller persists. What leaves here is unreadable anywhere but this machine
//! in this boot state, which is what makes it safe to hand to a capsule that
//! does hold the disk.
//!
//! Ownership is enforced exactly as export is. A blob is not a lower bar than
//! the plaintext: whoever holds it can bring the key back on this machine.

use alloc::vec::Vec;

use crate::protocol::{encode_response, Request, EACCES, EINVAL, ENOENT};
use crate::store::{Store, StoreError};
use crate::vault::{seal_secret, VaultError};

pub fn vault_seal(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
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
    let sealed = seal_secret(&secret);
    for b in secret.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    match sealed {
        Ok(blob) => encode_response(req.seq, 0, &blob),
        /*
         * No TPM, or a machine that is not in the state a key would belong
         * to. ENOENT rather than EINVAL: the request was well formed and the
         * caller can tell the user why nothing was saved.
         */
        Err(VaultError::NoKey) => encode_response(req.seq, ENOENT, &[]),
        Err(_) => encode_response(req.seq, EINVAL, &[]),
    }
}
