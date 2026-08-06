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

use super::sizes::{MATERIAL_BYTES, NONCE_BYTES};
use alloc::vec::Vec;

/// A sealed signature and the nonce it was sealed under.
pub struct Material {
    pub sealed: Vec<u8>,
    pub nonce: [u8; NONCE_BYTES],
}

impl Material {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(MATERIAL_BYTES);
        out.extend_from_slice(&self.sealed);
        out.extend_from_slice(&self.nonce);
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != MATERIAL_BYTES {
            return None;
        }
        let split = MATERIAL_BYTES - NONCE_BYTES;
        let mut nonce = [0u8; NONCE_BYTES];
        nonce.copy_from_slice(&bytes[split..]);
        Some(Self { sealed: bytes[..split].to_vec(), nonce })
    }
}
