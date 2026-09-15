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

//! Rebuilding the point R from the r of a signature.
//!
//! Signing throws away R and keeps its x coordinate. Recovery has to put it
//! back, which is solving the curve equation for y and then picking which of
//! the two roots the signer had.

use crate::field::FieldElement;
use crate::point::AffinePoint;

/*
 * The curve equation's constant. y^2 = x^3 + 7 over the field, so an x that
 * is on the curve has a square root here and one that is not has none.
 */
const B: FieldElement = FieldElement([7, 0, 0, 0]);

/// `None` when r is not a coordinate on the curve.
///
/// r arrives as a scalar, but the x coordinate of R is a field element, and
/// the two moduli differ. `from_bytes` refuses anything at or above p, so an r
/// that is a valid scalar but not a valid x is rejected rather than reduced
/// into a different point.
pub(crate) fn r_point(r_bytes: &[u8; 32], want_odd_y: bool) -> Option<AffinePoint> {
    let x = FieldElement::from_bytes(r_bytes)?;
    let y = x.mul(&x).mul(&x).add(&B).sqrt()?;

    /*
     * sqrt returns one of the two roots. The recovery id's low bit says which
     * of the pair the signer used, expressed as the parity of y.
     */
    let y = if want_odd_y == y.is_even() { y.negate() } else { y };
    Some(AffinePoint { x, y, infinity: false })
}
