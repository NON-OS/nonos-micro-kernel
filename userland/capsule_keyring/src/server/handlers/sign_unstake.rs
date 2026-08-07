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

use super::super::eip1559::{signed_nox_unstake_tx, unsigned_nox_unstake_payload};
use super::super::field32::field32;
use super::super::zeroize::zeroize32;
use crate::protocol::{encode_response, Request, EACCES, EINVAL};
use crate::store::{Store, StoreError};

// Sign unstakePosition(index) on the staking proxy. The contract closes a
// whole position by its index and returns its stake, so the figure carried
// here is a position, not an amount. Ownership-gated like every sign.
pub fn sign_unstake(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    const HDR: usize = 4 + 4 + 32 * 5;
    if req.payload.len() != HDR {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let payload_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let caller_pid = match crate::server::caller::resolve_caller(payload_pid, sender_pid) {
        Some(pid) => pid,
        None => return encode_response(req.seq, EACCES, &[]),
    };
    let id = u32::from_le_bytes([p[4], p[5], p[6], p[7]]);
    let mut secret = match store.eth_secret(id, caller_pid) {
        Ok(s) => s,
        Err(StoreError::AccessDenied) => return encode_response(req.seq, EACCES, &[]),
        Err(_) => return encode_response(req.seq, EINVAL, &[]),
    };
    let nonce = field32(p, 8);
    let max_priority = field32(p, 40);
    let max_fee = field32(p, 72);
    let gas = field32(p, 104);
    let index = field32(p, 136);
    let unsigned = unsigned_nox_unstake_payload(&nonce, &max_priority, &max_fee, &gas, &index);
    let mut digest = [0u8; 32];
    if nonos_libc::crypto_keccak256(unsigned.as_ptr(), unsigned.len(), digest.as_mut_ptr(), 32)
        != 32
    {
        zeroize32(&mut secret);
        return encode_response(req.seq, EINVAL, &[]);
    }
    let mut sig = [0u8; 65];
    let rc = nonos_libc::crypto_secp256k1_sign(secret.as_ptr(), digest.as_ptr(), sig.as_mut_ptr());
    zeroize32(&mut secret);
    if rc != 65 || sig[64] < 27 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let mut r = [0u8; 32];
    let mut s = [0u8; 32];
    r.copy_from_slice(&sig[0..32]);
    s.copy_from_slice(&sig[32..64]);
    let raw = signed_nox_unstake_tx(
        (&nonce, &max_priority, &max_fee, &gas, &index),
        sig[64] - 27,
        &r,
        &s,
    );
    encode_response(req.seq, 0, &raw)
}
