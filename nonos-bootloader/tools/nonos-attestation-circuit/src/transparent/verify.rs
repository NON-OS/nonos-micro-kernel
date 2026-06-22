// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::challenge::challenge;
use super::generator_h::generator_h;
use super::parse::parse;
use super::point::point;
use super::scalar_from::scalar_from;
use super::verify_membership::verify_membership;
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;

pub fn verify(root: &[u8; 32], ctx: &[u8], proof: &[u8]) -> Result<(), String> {
    let parsed = parse(proof)?;
    verify_membership(root, &parsed)?;
    let c = challenge(root, ctx, &parsed)?;
    let zx = scalar_from(parsed.z_x)?;
    let zr = scalar_from(parsed.z_r)?;
    let commitment = point(parsed.commitment)?;
    let nonce = point(parsed.nonce_point)?;
    let lhs = (ED25519_BASEPOINT_POINT * zx) + (generator_h() * zr);
    let rhs = nonce + (commitment * c);
    if lhs == rhs {
        Ok(())
    } else {
        Err("transparent equation failed".into())
    }
}
