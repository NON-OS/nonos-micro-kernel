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

use std::{fs, path::Path};

use super::constants::CAPSULE_ZK_MAGIC;
use super::input::ProofInput;
use super::read_u32::read_u32;
use super::take::take;

pub fn read_capsule(path: &Path) -> Result<ProofInput, String> {
    let data = fs::read(path).map_err(|e| format!("read capsule: {e}"))?;
    let offset = data
        .windows(CAPSULE_ZK_MAGIC.len())
        .rposition(|w| w == CAPSULE_ZK_MAGIC)
        .ok_or("capsule ZK trailer not found")?;
    let mut pos = offset + CAPSULE_ZK_MAGIC.len();
    let proof_len = read_u32(&data, &mut pos)? as usize;
    let proof = take(&data, &mut pos, proof_len)?.to_vec();
    let public_input_len = read_u32(&data, &mut pos)? as usize;
    let public_inputs = take(&data, &mut pos, public_input_len)?.to_vec();
    let commit = take(&data, &mut pos, 32)?;
    if pos != data.len() {
        return Err("trailing bytes after capsule ZK trailer".into());
    }
    let mut trailer_commitment = [0u8; 32];
    trailer_commitment.copy_from_slice(commit);
    let body_hash = *blake3::hash(&data[..offset]).as_bytes();
    Ok(ProofInput {
        proof,
        public_inputs,
        trailer_commitment: Some(trailer_commitment),
        body_hash: Some(body_hash),
    })
}
