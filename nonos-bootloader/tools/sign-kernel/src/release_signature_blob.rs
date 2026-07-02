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

use crate::constants::{RELEASE_SIG_MAGIC, RELEASE_SIG_SIZE};
use crate::key_id_ed25519::ed25519_key_id;
use crate::key_id_mldsa65::mldsa65_key_id;

pub fn release_signature_blob(
    ed_pub: &[u8; 32],
    ed_sig: &[u8; 64],
    pq_pub: &[u8],
    pq_sig: &[u8],
) -> Vec<u8> {
    let mut out = Vec::with_capacity(RELEASE_SIG_SIZE);
    out.extend_from_slice(&RELEASE_SIG_MAGIC);
    out.extend_from_slice(&ed25519_key_id(ed_pub));
    out.extend_from_slice(ed_sig);
    out.extend_from_slice(&mldsa65_key_id(pq_pub));
    out.extend_from_slice(pq_sig);
    out
}
