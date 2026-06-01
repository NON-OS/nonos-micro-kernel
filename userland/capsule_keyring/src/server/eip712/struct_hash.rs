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

use super::consts::RECEIPT_TYPEHASH;
use super::fields::ReceiptFields;

pub fn struct_hash(f: &ReceiptFields) -> Option<[u8; 32]> {
    let mut pre = [0u8; 288];
    pre[0..32].copy_from_slice(&RECEIPT_TYPEHASH);
    pre[32..64].copy_from_slice(&f.capsule_id);
    pre[76..96].copy_from_slice(&f.user);
    pre[108..128].copy_from_slice(&f.publisher);
    pre[128..160].copy_from_slice(&f.amount_nox);
    pre[160..192].copy_from_slice(&f.nonce);
    pre[192..224].copy_from_slice(&f.epoch);
    pre[224..256].copy_from_slice(&f.expiry);
    pre[256..288].copy_from_slice(&f.receipt_type);
    let mut out = [0u8; 32];
    if nonos_libc::crypto_keccak256(pre.as_ptr(), 288, out.as_mut_ptr(), 32) != 32 {
        return None;
    }
    Some(out)
}
