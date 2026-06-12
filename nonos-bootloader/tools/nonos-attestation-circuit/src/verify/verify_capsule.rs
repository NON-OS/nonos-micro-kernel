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

use std::path::Path;

use crate::constants::GROTH16_PROOF_SIZE;

use super::binding::binding;
use super::parse_public_inputs::parse_public_inputs;
use super::read_capsule::read_capsule;
use super::read_vk::read_vk;
use super::verify_groth16::verify_groth16;

pub fn verify_capsule(vk_path: &Path, capsule_path: &Path) -> Result<(), String> {
    let vk = read_vk(vk_path)?;
    let input = read_capsule(capsule_path)?;
    if input.proof.len() != GROTH16_PROOF_SIZE {
        return Err(format!("proof size {} != {}", input.proof.len(), GROTH16_PROOF_SIZE));
    }
    let fields = parse_public_inputs(&input.public_inputs)?;
    binding(&input.public_inputs, input.body_hash, input.trailer_commitment)?;
    if !verify_groth16(&vk, &input.proof, &fields)? {
        return Err("groth16 verification failed".into());
    }
    Ok(())
}
