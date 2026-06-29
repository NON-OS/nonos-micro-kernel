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

use alloc::vec;
use alloc::vec::Vec;

const MAGIC: u32 = 0x4e4f_4358;
const OP_RSA_VERIFY: u16 = 21;

pub fn verify_rsa(scheme: u8, hashid: u8, spki: &[u8], sig: &[u8], digest: &[u8]) -> bool {
    let Some(port) = super::crypto_port::crypto_port() else {
        return false;
    };
    let mut body: Vec<u8> = Vec::with_capacity(6 + spki.len() + sig.len() + digest.len());
    body.push(scheme);
    body.push(hashid);
    body.extend_from_slice(&(spki.len() as u16).to_le_bytes());
    body.extend_from_slice(spki);
    body.extend_from_slice(&(sig.len() as u16).to_le_bytes());
    body.extend_from_slice(sig);
    body.extend_from_slice(digest);
    let mut tx = vec![0u8; 20 + body.len()];
    let mut rx = [0u8; 32];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_RSA_VERIFY.to_le_bytes());
    tx[12..16].copy_from_slice(&7u32.to_le_bytes());
    tx[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    tx[20..].copy_from_slice(&body);
    let rc = nonos_libc::mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    rc >= 24
        && u32::from_le_bytes([rx[0], rx[1], rx[2], rx[3]]) == MAGIC
        && u16::from_le_bytes([rx[6], rx[7]]) == OP_RSA_VERIFY
        && i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]) == 0
}
