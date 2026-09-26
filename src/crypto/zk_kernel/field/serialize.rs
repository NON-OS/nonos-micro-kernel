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

use super::types::FieldElement;
use crate::crypto::asymmetric::ed25519::sc_reduce_mod_l;
use crate::crypto::rng::fill_random_bytes;

impl FieldElement {
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let mut wide = [0u8; 64];
        wide[..32].copy_from_slice(bytes);
        Self { bytes: sc_reduce_mod_l(&mut wide) }
    }

    /// A 64-byte value reduced mod L by the same routine `from_bytes` uses.
    pub fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        let mut wide = *bytes;
        Self { bytes: sc_reduce_mod_l(&mut wide) }
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.bytes
    }

    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        fill_random_bytes(&mut bytes);
        bytes[31] &= 0x0F;
        Self::from_bytes(&bytes)
    }
}
