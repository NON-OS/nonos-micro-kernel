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

use super::super::{FieldElement, Scalar};

#[derive(Clone)]
pub struct AffinePoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub infinity: bool,
}

#[derive(Clone)]
pub struct ProjectivePoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
}

impl AffinePoint {
    pub fn scalar_mul(&self, k: &Scalar) -> AffinePoint {
        let mut result =
            AffinePoint { x: FieldElement::ZERO, y: FieldElement::ZERO, infinity: true };

        let k_bytes = k.to_bytes();
        for i in (0..32).rev() {
            for j in (0..8).rev() {
                result = result.double();
                if (k_bytes[i] >> j) & 1 == 1 {
                    result = result.add(self);
                }
            }
        }

        result
    }

    fn double(&self) -> AffinePoint {
        if self.infinity {
            return self.clone();
        }
        self.clone()
    }

    fn add(&self, _other: &AffinePoint) -> AffinePoint {
        self.clone()
    }
}
