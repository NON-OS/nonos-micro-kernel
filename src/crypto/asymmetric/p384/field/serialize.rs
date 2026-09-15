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

use super::compare::ct_lt_u64;
use super::types::FieldElement;

const LIMBS: usize = 6;
const BYTES: usize = 48;

impl FieldElement {
    pub fn from_bytes(bytes: &[u8; BYTES]) -> Option<Self> {
        let mut limbs = [0u64; LIMBS];
        for (i, limb) in limbs.iter_mut().enumerate() {
            let offset = (LIMBS - 1 - i) * 8;
            *limb = u64::from_be_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
                bytes[offset + 4],
                bytes[offset + 5],
                bytes[offset + 6],
                bytes[offset + 7],
            ]);
        }
        let fe = Self(limbs);
        if fe.is_valid() {
            Some(fe)
        } else {
            None
        }
    }

    pub fn to_bytes(&self) -> [u8; BYTES] {
        let mut bytes = [0u8; BYTES];
        for i in 0..LIMBS {
            let offset = (LIMBS - 1 - i) * 8;
            let limb_bytes = self.0[i].to_be_bytes();
            bytes[offset..offset + 8].copy_from_slice(&limb_bytes);
        }
        bytes
    }

    fn is_valid(&self) -> bool {
        let mut lt: u64 = 0;
        let mut eq: u64 = 1;
        for i in (0..LIMBS).rev() {
            let a = self.0[i];
            let p = Self::P[i];
            let a_lt_p = ct_lt_u64(a, p);
            let a_gt_p = ct_lt_u64(p, a);
            lt |= eq & a_lt_p;
            eq &= (1 - a_lt_p) & (1 - a_gt_p);
        }
        lt == 1
    }
}
