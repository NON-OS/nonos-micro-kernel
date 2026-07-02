// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use uefi::cstr16;
use uefi::prelude::*;

use super::display::print;
use crate::crypto::keys::{constant_time_eq, get_nonos_key_id};
use crate::crypto::mldsa65;

pub const RELEASE_SIG_MAGIC: [u8; 8] = *b"NKRSIG2\0";
pub const ED_SIG_START: usize = 40;
pub const PQ_ID_START: usize = 104;
pub const PQ_SIG_START: usize = 136;

pub fn policy_identity_ok(signature: &[u8], st: &mut SystemTable<Boot>) -> bool {
    if signature.len() < PQ_SIG_START || signature[0..8] != RELEASE_SIG_MAGIC {
        print(st, cstr16!("  [CRYPTO] Kernel release signature bundle invalid [FAIL]\r\n"));
        return false;
    }
    let ed_id = array32(&signature[8..ED_SIG_START]);
    let pq_id = array32(&signature[PQ_ID_START..PQ_SIG_START]);
    if !constant_time_eq(&ed_id, get_nonos_key_id()) {
        print(st, cstr16!("  [CRYPTO] Required Ed25519 signer mismatch ..... [FAIL]\r\n"));
        return false;
    }
    if !constant_time_eq(&pq_id, &mldsa65::trusted_key_id()) {
        print(st, cstr16!("  [CRYPTO] Required ML-DSA signer mismatch ...... [FAIL]\r\n"));
        return false;
    }
    true
}

fn array32(bytes: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    out.copy_from_slice(bytes);
    out
}
