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

use nonos_libc::mk_ipc_call;

use super::consts::KEYRING_OP_SIGN_RECEIPT;
use super::fields::{ReceiptInput, SignedReceipt};

pub fn sign_receipt(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    f: &ReceiptInput,
) -> Result<SignedReceipt, i32> {
    let mut tx: Vec<u8> = Vec::with_capacity(8 + 220);
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&KEYRING_OP_SIGN_RECEIPT.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&owner_pid.to_le_bytes());
    tx.extend_from_slice(&wallet_id.to_le_bytes());
    tx.extend_from_slice(&f.capsule_id);
    tx.extend_from_slice(&f.publisher);
    tx.extend_from_slice(&f.amount);
    tx.extend_from_slice(&f.nonce);
    tx.extend_from_slice(&f.epoch);
    tx.extend_from_slice(&f.expiry);
    tx.extend_from_slice(&f.receipt_type);
    let mut rx = vec![0u8; 8 + 117];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (8 + 117) as i64 {
        return Err(-11);
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return Err(status);
    }
    let mut out = SignedReceipt { user: [0u8; 20], struct_hash: [0u8; 32], signature: [0u8; 65] };
    out.user.copy_from_slice(&rx[8..28]);
    out.struct_hash.copy_from_slice(&rx[28..60]);
    out.signature.copy_from_slice(&rx[60..125]);
    Ok(out)
}
