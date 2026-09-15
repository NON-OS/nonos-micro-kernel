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

use super::super::field::FieldElement;
use super::super::scalar::Scalar;
use super::types::{AffinePoint, ProjectivePoint};

impl ProjectivePoint {
    pub fn identity() -> Self {
        Self { x: FieldElement::ZERO, y: FieldElement::ONE, z: FieldElement::ZERO }
    }

    pub fn is_identity(&self) -> bool {
        self.z.0 == [0, 0, 0, 0]
    }

    pub fn double(&self) -> Self {
        let xx = self.x.square();
        let yy = self.y.square();
        let yyyy = yy.square();
        let zz = self.z.square();

        let s = self.x.add(&yy).square().sub(&xx).sub(&yyyy);
        let s = s.add(&s);

        let m = xx.add(&xx).add(&xx);

        let t = m.square().sub(&s).sub(&s);

        let x3 = t.clone();
        let y3 = m.mul(&s.sub(&t)).sub(
            &yyyy.add(&yyyy).add(&yyyy).add(&yyyy).add(&yyyy).add(&yyyy).add(&yyyy).add(&yyyy),
        );
        let z3 = self.y.add(&self.z).square().sub(&yy).sub(&zz);

        let result = Self { x: x3, y: y3, z: z3 };

        let is_id = self.z.ct_is_zero();
        Self::ct_select(is_id, self, &result)
    }

    pub fn add(&self, other: &Self) -> Self {
        let z1z1 = self.z.square();
        let z2z2 = other.z.square();
        let u1 = self.x.mul(&z2z2);
        let u2 = other.x.mul(&z1z1);
        let s1 = self.y.mul(&other.z).mul(&z2z2);
        let s2 = other.y.mul(&self.z).mul(&z1z1);

        let h = u2.sub(&u1);
        let i = h.add(&h).square();
        let j = h.mul(&i);
        let r = s2.sub(&s1).add(&s2.sub(&s1));
        let v = u1.mul(&i);

        let x3 = r.square().sub(&j).sub(&v).sub(&v);
        let y3 = r.mul(&v.sub(&x3)).sub(&s1.mul(&j).add(&s1.mul(&j)));
        let z3 = self.z.add(&other.z).square().sub(&z1z1).sub(&z2z2).mul(&h);

        let add_result = Self { x: x3, y: y3, z: z3 };

        let double_result = self.double();

        let self_is_id = self.z.ct_is_zero();
        let other_is_id = other.z.ct_is_zero();
        let u_eq = u1.ct_eq(&u2);
        let s_eq = s1.ct_eq(&s2);

        let mut result = Self::ct_select(self_is_id, other, &add_result);
        result = Self::ct_select(other_is_id, self, &result);
        result = Self::ct_select(u_eq & s_eq, &double_result, &result);
        let return_identity = u_eq & (1 ^ s_eq) & (1 ^ self_is_id) & (1 ^ other_is_id);
        result = Self::ct_select(return_identity, &Self::identity(), &result);

        result
    }

    fn ct_select(condition: u64, a: &Self, b: &Self) -> Self {
        let mask = 0u64.wrapping_sub(condition);
        Self {
            x: FieldElement::ct_select(mask, &a.x, &b.x),
            y: FieldElement::ct_select(mask, &a.y, &b.y),
            z: FieldElement::ct_select(mask, &a.z, &b.z),
        }
    }

    pub fn mul(&self, scalar: &Scalar) -> Self {
        let mut r0 = Self::identity();
        let mut r1 = self.clone();

        for i in (0..256).rev() {
            let limb_idx = i / 64;
            let bit_idx = i % 64;
            let bit = (scalar.0[limb_idx] >> bit_idx) & 1;

            let sum = r0.add(&r1);
            let r0_double = r0.double();
            let r1_double = r1.double();

            r0 = Self::ct_select(bit, &sum, &r0_double);
            r1 = Self::ct_select(bit, &r1_double, &sum);
        }

        r0
    }

    pub fn to_affine(&self) -> AffinePoint {
        if self.is_identity() {
            return AffinePoint::identity();
        }

        let z_inv = match self.z.invert() {
            Some(inv) => inv,
            None => return AffinePoint::identity(),
        };
        let z_inv2 = z_inv.square();
        let z_inv3 = z_inv2.mul(&z_inv);

        AffinePoint { x: self.x.mul(&z_inv2), y: self.y.mul(&z_inv3), infinity: false }
    }
}
