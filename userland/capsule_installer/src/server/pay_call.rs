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

use super::consts::PAYMENT_OP_PAY;
use super::fields::InstallReq;

pub fn request_payment(port: u32, r: &InstallReq) -> Result<[u8; 32], i32> {
    let mut tx: Vec<u8> = Vec::with_capacity(8 + 124);
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&PAYMENT_OP_PAY.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&r.owner_pid.to_le_bytes());
    tx.extend_from_slice(&r.wallet_id.to_le_bytes());
    tx.extend_from_slice(&r.capsule_id);
    tx.extend_from_slice(&r.publisher);
    tx.extend_from_slice(&r.amount);
    tx.extend_from_slice(&r.receipt_type);
    let mut rx = vec![0u8; 8 + 32];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (8 + 32) as i64 {
        return Err(-11);
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return Err(status);
    }
    let mut sh = [0u8; 32];
    sh.copy_from_slice(&rx[8..40]);
    Ok(sh)
}
