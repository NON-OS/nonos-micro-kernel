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

//! The public key a secret derives.

use crate::point::AffinePoint;
use crate::scalar::Scalar;
use crate::{PublicKey, SecretKey};

/// `None` when the bytes are not a key.
///
/// Zero is refused explicitly. `Scalar::from_bytes` only checks that the value
/// is below the group order, and zero is, so it used to be accepted: the
/// multiplication then produced the point at infinity and `to_uncompressed`
/// encoded it as `04` followed by sixty-four zero bytes. That is not a public
/// key, but it looks exactly like one to every caller, and an address derived
/// from it is a real address nobody holds the key to. Funds sent there are
/// gone. The infinity check afterwards is the same refusal reached from the
/// other side, kept so no future edit to the scalar check can reopen it.
pub fn public_key_from_secret(sk: &SecretKey) -> Option<PublicKey> {
    let scalar = Scalar::from_bytes(sk)?;
    if scalar.is_zero() {
        return None;
    }
    let point = AffinePoint::generator().to_projective().mul(&scalar).to_affine();
    if point.infinity {
        return None;
    }
    Some(point.to_uncompressed())
}
