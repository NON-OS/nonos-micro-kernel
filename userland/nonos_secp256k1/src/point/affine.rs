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
use super::types::{AffinePoint, ProjectivePoint};

impl AffinePoint {
    pub fn identity() -> Self {
        Self { x: FieldElement::ZERO, y: FieldElement::ZERO, infinity: true }
    }

    pub fn generator() -> Self {
        Self {
            x: FieldElement([
                0x59F2815B16F81798,
                0x029BFCDB2DCE28D9,
                0x55A06295CE870B07,
                0x79BE667EF9DCBBAC,
            ]),
            y: FieldElement([
                0x9C47D08FFB10D4B8,
                0xFD17B448A6855419,
                0x5DA4FBFC0E1108A8,
                0x483ADA7726A3C465,
            ]),
            infinity: false,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes.len() {
            33 => Self::from_compressed(bytes.try_into().ok()?),
            65 => Self::from_uncompressed(bytes.try_into().ok()?),
            _ => None,
        }
    }

    pub fn from_compressed(bytes: &[u8; 33]) -> Option<Self> {
        if bytes[0] != 0x02 && bytes[0] != 0x03 {
            return None;
        }

        let x = FieldElement::from_bytes(bytes[1..33].try_into().ok()?)?;

        if x.0 == [0, 0, 0, 0] {
            return None;
        }

        let y_squared = x.mul(&x).mul(&x).add(&FieldElement([7, 0, 0, 0]));
        let y = y_squared.sqrt()?;

        let y = if (bytes[0] == 0x02) == y.is_even() { y } else { y.negate() };

        Some(Self { x, y, infinity: false })
    }

    pub fn from_uncompressed(bytes: &[u8; 65]) -> Option<Self> {
        if bytes[0] != 0x04 {
            return None;
        }

        let x = FieldElement::from_bytes(bytes[1..33].try_into().ok()?)?;
        let y = FieldElement::from_bytes(bytes[33..65].try_into().ok()?)?;

        if x.0 == [0, 0, 0, 0] && y.0 == [0, 0, 0, 0] {
            return None;
        }

        let y_squared = x.mul(&x).mul(&x).add(&FieldElement([7, 0, 0, 0]));
        if y.mul(&y) != y_squared {
            return None;
        }

        Some(Self { x, y, infinity: false })
    }

    pub fn to_uncompressed(&self) -> [u8; 65] {
        let mut bytes = [0u8; 65];
        bytes[0] = 0x04;
        bytes[1..33].copy_from_slice(&self.x.to_bytes());
        bytes[33..65].copy_from_slice(&self.y.to_bytes());
        bytes
    }

    pub fn to_compressed(&self) -> [u8; 33] {
        let mut bytes = [0u8; 33];
        bytes[0] = if self.y.is_even() { 0x02 } else { 0x03 };
        bytes[1..33].copy_from_slice(&self.x.to_bytes());
        bytes
    }

    pub fn to_projective(&self) -> ProjectivePoint {
        if self.infinity {
            ProjectivePoint { x: FieldElement::ZERO, y: FieldElement::ONE, z: FieldElement::ZERO }
        } else {
            ProjectivePoint { x: self.x.clone(), y: self.y.clone(), z: FieldElement::ONE }
        }
    }
}
