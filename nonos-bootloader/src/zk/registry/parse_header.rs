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

use super::types_section::{CircuitSectionHeader, CIRCUIT_SECTION_MAGIC};
use core::mem::size_of;

pub fn validate_header(section: &[u8]) -> Result<(u32, usize, [u8; 64], [u8; 32]), &'static str> {
    if section.len() < size_of::<CircuitSectionHeader>() {
        return Err("circuit: section too small");
    }
    if &section[0..4] != CIRCUIT_SECTION_MAGIC {
        return Err("circuit: invalid magic");
    }
    let version =
        u32::from_le_bytes(section[4..8].try_into().map_err(|_| "circuit: version parse failed")?);
    if version != 1 {
        return Err("circuit: unsupported version");
    }
    let count =
        u32::from_le_bytes(section[8..12].try_into().map_err(|_| "circuit: count parse failed")?)
            as usize;
    let mut signature = [0u8; 64];
    signature.copy_from_slice(&section[16..80]);
    let mut signer = [0u8; 32];
    signer.copy_from_slice(&section[80..112]);
    Ok((version, count, signature, signer))
}
