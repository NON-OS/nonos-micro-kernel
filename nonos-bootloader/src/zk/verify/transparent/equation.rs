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
use super::ctx::ctx;
use super::generator_h::generator_h;
use super::input_types::BootTransparentInputs;
use super::point::point;
use super::scalar::scalar;
use super::types::TransparentProof;
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;

pub fn equation(
    root: &[u8; 32],
    inputs: &BootTransparentInputs,
    proof: &TransparentProof<'_>,
) -> Result<(), &'static str> {
    let c = challenge(root, &ctx(root, inputs), proof)?;
    let zx = scalar(proof.z_x)?;
    let zr = scalar(proof.z_r)?;
    let commitment = point(proof.commitment)?;
    let nonce = point(proof.nonce_point)?;
    let lhs = (ED25519_BASEPOINT_POINT * zx) + (generator_h() * zr);
    let rhs = nonce + (commitment * c);
    if lhs == rhs {
        Ok(())
    } else {
        Err("transparent equation failed")
    }
}
