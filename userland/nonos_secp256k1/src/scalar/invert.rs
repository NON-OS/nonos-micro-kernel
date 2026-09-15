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

use super::types::Scalar;

impl Scalar {
    pub fn invert(&self) -> Option<Self> {
        let n_minus_2: [u64; 4] =
            [0xBFD25E8CD036413F, 0xBAAEDCE6AF48A03B, 0xFFFFFFFFFFFFFFFE, 0xFFFFFFFFFFFFFFFF];

        let mut result = Self::ONE;
        let mut base = self.clone();

        for &limb in n_minus_2.iter() {
            for bit in 0..64 {
                let mul_result = result.mul(&base);
                let mask = 0u64.wrapping_sub((limb >> bit) & 1);
                result = Self::ct_select(mask, &mul_result, &result);
                base = base.mul(&base);
            }
        }

        let is_zero = self.ct_is_zero();
        if is_zero == 1 {
            None
        } else {
            Some(result)
        }
    }
}
