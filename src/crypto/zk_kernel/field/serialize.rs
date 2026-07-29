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

    // SECURITY: Constant-time Barrett reduction for 64-byte input.
    pub fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        const MU: [u8; 32] = [
            0x1d, 0x95, 0x98, 0x4d, 0x74, 0x31, 0xec, 0xd6, 0x70, 0xcf, 0x7d, 0x73, 0xf4, 0x5b,
            0xef, 0xc6, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0x0f,
        ];

        let mut acc = [0u32; 64];

        for i in 0..32 {
            acc[i] += bytes[i] as u32;
        }

        for i in 0..32 {
            let hi = bytes[32 + i] as u32;
            for j in 0..32 {
                acc[i + j] += hi * (MU[j] as u32);
            }
        }

        for i in 0..63 {
            acc[i + 1] += acc[i] >> 8;
            acc[i] &= 0xFF;
        }

        let mut result = [0u8; 32];
        for i in 0..32 {
            result[i] = acc[i] as u8;
        }

        let mut high_acc = [0u32; 64];
        for i in 32..64 {
            let hi = acc[i] as u32;
            for j in 0..32 {
                high_acc[i - 32 + j] += hi * (MU[j] as u32);
            }
        }

        for i in 0..63 {
            high_acc[i + 1] += high_acc[i] >> 8;
            high_acc[i] &= 0xFF;
        }

        let mut extra = [0u8; 32];
        for i in 0..32 {
            extra[i] = high_acc[i] as u8;
        }

        let mut carry: u16 = 0;
        for i in 0..32 {
            let sum = result[i] as u16 + extra[i] as u16 + carry;
            result[i] = sum as u8;
            carry = sum >> 8;
        }

        let mut fe = Self { bytes: result };
        fe.reduce();
        fe
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
