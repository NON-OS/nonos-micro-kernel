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

//! The byte forms a caller sees.

pub type SecretKey = [u8; 32];
pub type PublicKey = [u8; 65];
pub type CompressedPublicKey = [u8; 33];
pub type Signature = [u8; 64];

#[derive(Debug, Clone, Copy)]
pub struct RecoverableSignature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub recovery_id: u8,
}

impl RecoverableSignature {
    pub fn from_bytes(bytes: &[u8; 65]) -> Self {
        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        r.copy_from_slice(&bytes[0..32]);
        s.copy_from_slice(&bytes[32..64]);
        Self { r, s, recovery_id: bytes[64] }
    }

    pub fn to_bytes(&self) -> [u8; 65] {
        let mut bytes = [0u8; 65];
        bytes[0..32].copy_from_slice(&self.r);
        bytes[32..64].copy_from_slice(&self.s);
        bytes[64] = self.recovery_id;
        bytes
    }
}

/*
 * The kernel module also carried multiply_point, point_add and
 * scalar_multiply, described as ECDH helpers. None of them had a caller
 * anywhere in the tree, so they are not carried across: dead code in a
 * crypto library is surface with no test and no reader.
 */
