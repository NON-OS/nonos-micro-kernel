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

use crate::crypto::hmac_sha256;

pub fn make(
    owner: u32,
    session: u32,
    id: u32,
    cred: &[u8; 32],
    seed: &[u8; 32],
) -> Option<[u8; 32]> {
    let mut material = Vec::with_capacity(44);
    material.extend_from_slice(&owner.to_le_bytes());
    material.extend_from_slice(&session.to_le_bytes());
    material.extend_from_slice(&id.to_le_bytes());
    material.extend_from_slice(seed);
    let mut out = [0u8; 32];
    hmac_sha256(cred, &material, &mut out).ok()?;
    Some(out)
}

pub fn matches(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}
