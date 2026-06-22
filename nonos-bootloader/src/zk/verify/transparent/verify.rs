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

use super::device_root::device_root;
use super::equation::equation;
use super::freshness::freshness;
use super::merkle::merkle_root;
use super::parse::parse;
use super::public_inputs::public_inputs;
use crate::zk::verify::util::ct_eq32;

pub fn verify_transparent(
    root: &[u8; 32],
    public: &[u8],
    proof_blob: &[u8],
) -> Result<(), &'static str> {
    let inputs = public_inputs(public)?;
    freshness(&inputs)?;
    if !ct_eq32(root, &device_root()?) {
        return Err("transparent device root mismatch");
    }
    let proof = parse(proof_blob)?;
    let folded = merkle_root(&proof)?;
    if !ct_eq32(&folded, root) {
        return Err("transparent merkle root mismatch");
    }
    equation(root, &inputs, &proof)
}
