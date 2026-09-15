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

impl FieldElement {
    pub fn pow(&self, exp: &[u64; 4]) -> Self {
        let mut result = Self::ONE;
        let mut base = self.clone();

        for &limb in exp.iter() {
            for bit in 0..64 {
                let mul_result = result.mul(&base);
                let mask = 0u64.wrapping_sub((limb >> bit) & 1);
                result = Self::ct_select(mask, &mul_result, &result);
                base = base.square();
            }
        }
        result
    }

    pub fn invert(&self) -> Option<Self> {
        let exp = [0xFFFFFFFFFFFFFFFD, 0x00000000FFFFFFFF, 0x0000000000000000, 0xFFFFFFFF00000001];
        let result = self.pow(&exp);

        let is_zero = self.ct_is_zero();
        if is_zero == 1 {
            None
        } else {
            Some(result)
        }
    }

    pub fn sqrt(&self) -> Option<Self> {
        let exp = [0x0000000000000000, 0x4000000040000000, 0x0000000000000000, 0x3FFFFFFFC0000000];
        let r = self.pow(&exp);
        let r_squared = r.square();
        let is_valid = r_squared.ct_eq(self);
        if is_valid == 1 {
            Some(r)
        } else {
            None
        }
    }
}
