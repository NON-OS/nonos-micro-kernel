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
use super::commitment::commitment;
use super::derive_scalar::derive_scalar;
use super::generator_h::generator_h;
use super::pack_dirs::pack_dirs;
use super::parse::parse;
use super::serialize::serialize;
use super::types::EnrolledSecret;
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;

pub fn prove(
    secret: &EnrolledSecret,
    siblings: &[[u8; 32]],
    dirs: &[u8],
    root: &[u8; 32],
    ctx: &[u8],
    nonce_seed: &[u8],
) -> Result<Vec<u8>, String> {
    let tx = derive_scalar(b"NONOS:TRANSPARENT:TX:v1", nonce_seed);
    let tr = derive_scalar(b"NONOS:TRANSPARENT:TR:v1", nonce_seed);
    let c_bytes = commitment(secret);
    let a_bytes = ((ED25519_BASEPOINT_POINT * tx) + (generator_h() * tr)).compress().to_bytes();
    let packed = pack_dirs(dirs);
    let skeleton = serialize(&c_bytes, &a_bytes, &[0u8; 32], &[0u8; 32], siblings, dirs);
    let parsed = parse(&skeleton[0..129 + siblings.len() * 32 + packed.len()])?;
    let chal = challenge(root, ctx, &parsed)?;
    let zx = tx + chal * secret.x;
    let zr = tr + chal * secret.r;
    Ok(serialize(&c_bytes, &a_bytes, &zx.to_bytes(), &zr.to_bytes(), siblings, dirs))
}
