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

use super::super::FieldElement;
use super::constants::{D, D2};

#[derive(Clone)]
pub struct EdwardsPoint {
    pub(crate) x: FieldElement,
    pub(crate) y: FieldElement,
    pub(crate) z: FieldElement,
    pub(crate) t: FieldElement,
}

impl Drop for EdwardsPoint {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl EdwardsPoint {
    pub fn identity() -> Self {
        EdwardsPoint {
            x: FieldElement::zero(),
            y: FieldElement::one(),
            z: FieldElement::one(),
            t: FieldElement::zero(),
        }
    }

    pub fn add(&self, other: &EdwardsPoint) -> EdwardsPoint {
        let a = self.y.sub(&self.x).mul(&other.y.sub(&other.x));
        let b = self.y.add(&self.x).mul(&other.y.add(&other.x));
        let c = self.t.mul(&D2).mul(&other.t);
        let d = self.z.mul(&other.z);
        let d = d.add(&d);
        let e = b.sub(&a);
        let f = d.sub(&c);
        let g = d.add(&c);
        let h = b.add(&a);

        EdwardsPoint { x: e.mul(&f), y: g.mul(&h), z: f.mul(&g), t: e.mul(&h) }
    }

    pub fn double(&self) -> EdwardsPoint {
        let a = self.x.square();
        let b = self.y.square();
        let c = self.z.square();
        let c = c.add(&c);
        let d = a.neg();
        let e = self.x.add(&self.y).square().sub(&a).sub(&b);
        let g = d.add(&b);
        let f = g.sub(&c);
        let h = d.sub(&b);

        EdwardsPoint { x: e.mul(&f), y: g.mul(&h), z: f.mul(&g), t: e.mul(&h) }
    }

    pub fn scalar_mul(&self, scalar: &[u8; 32]) -> EdwardsPoint {
        let mut r0 = EdwardsPoint::identity();
        let mut r1 = self.clone();

        for i in (0..256).rev() {
            let byte = i / 8;
            let bit = i % 8;
            let b = ((scalar[byte] >> bit) & 1) as u64;

            let sum = r0.add(&r1);
            let r0_double = r0.double();
            let r1_double = r1.double();

            r0 = Self::ct_select(b, &sum, &r0_double);
            r1 = Self::ct_select(b, &r1_double, &sum);
        }

        r0
    }

    fn ct_select(condition: u64, a: &Self, b: &Self) -> Self {
        let mask = 0u64.wrapping_sub(condition);
        let inv_mask = !mask;
        Self {
            x: FieldElement([
                (a.x.0[0] & mask) | (b.x.0[0] & inv_mask),
                (a.x.0[1] & mask) | (b.x.0[1] & inv_mask),
                (a.x.0[2] & mask) | (b.x.0[2] & inv_mask),
                (a.x.0[3] & mask) | (b.x.0[3] & inv_mask),
                (a.x.0[4] & mask) | (b.x.0[4] & inv_mask),
            ]),
            y: FieldElement([
                (a.y.0[0] & mask) | (b.y.0[0] & inv_mask),
                (a.y.0[1] & mask) | (b.y.0[1] & inv_mask),
                (a.y.0[2] & mask) | (b.y.0[2] & inv_mask),
                (a.y.0[3] & mask) | (b.y.0[3] & inv_mask),
                (a.y.0[4] & mask) | (b.y.0[4] & inv_mask),
            ]),
            z: FieldElement([
                (a.z.0[0] & mask) | (b.z.0[0] & inv_mask),
                (a.z.0[1] & mask) | (b.z.0[1] & inv_mask),
                (a.z.0[2] & mask) | (b.z.0[2] & inv_mask),
                (a.z.0[3] & mask) | (b.z.0[3] & inv_mask),
                (a.z.0[4] & mask) | (b.z.0[4] & inv_mask),
            ]),
            t: FieldElement([
                (a.t.0[0] & mask) | (b.t.0[0] & inv_mask),
                (a.t.0[1] & mask) | (b.t.0[1] & inv_mask),
                (a.t.0[2] & mask) | (b.t.0[2] & inv_mask),
                (a.t.0[3] & mask) | (b.t.0[3] & inv_mask),
                (a.t.0[4] & mask) | (b.t.0[4] & inv_mask),
            ]),
        }
    }

    pub fn compress(&self) -> [u8; 32] {
        let z_inv = self.z.invert();
        let x = self.x.mul(&z_inv);
        let y = self.y.mul(&z_inv);

        let mut bytes = y.to_bytes();
        bytes[31] ^= (x.is_negative() as u8) << 7;
        bytes
    }

    pub fn decompress(bytes: &[u8; 32]) -> Option<EdwardsPoint> {
        let mut y_bytes = *bytes;
        let x_sign = (y_bytes[31] >> 7) & 1;
        y_bytes[31] &= 0x7f;

        let y = FieldElement::from_bytes(&y_bytes);
        let y2 = y.square();

        let num = y2.sub(&FieldElement::one());
        let den = D.mul(&y2).add(&FieldElement::one());
        let den_inv = den.invert();
        let x2 = num.mul(&den_inv);

        let x = x2.sqrt()?;

        let x = if (x.is_negative() as u8) != x_sign { x.neg() } else { x };

        let t = x.mul(&y);
        Some(EdwardsPoint { x, y, z: FieldElement::one(), t })
    }

    pub fn zeroize(&mut self) {
        self.x.zeroize();
        self.y.zeroize();
        self.z.zeroize();
        self.t.zeroize();
    }

    pub fn negate(&self) -> EdwardsPoint {
        EdwardsPoint { x: self.x.neg(), y: self.y.clone(), z: self.z.clone(), t: self.t.neg() }
    }

    pub fn is_identity(&self) -> bool {
        let z_inv = self.z.invert();
        let x_normalized = self.x.mul(&z_inv);
        let y_normalized = self.y.mul(&z_inv);

        x_normalized.is_zero() && y_normalized.equals(&FieldElement::one())
    }
}
