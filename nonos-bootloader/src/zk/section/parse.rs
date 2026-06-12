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

use core::mem::size_of;

use crate::zk::errors::ZkError;
use crate::zk::verify::ZkProof;

/// Raw section header offsets
#[repr(C)]
struct RawHeader {
    program_hash_off: u32,
    capsule_commitment_off: u32,
    public_inputs_off: u32,
    proof_off: u32,
    end_off: u32,
}

/// Read u32 from section at offset
fn read_u32(section: &[u8], off: usize) -> Result<u32, ZkError> {
    if off + 4 > section.len() {
        return Err(ZkError::HeaderTruncated);
    }
    Ok(u32::from_le_bytes(section[off..off + 4].try_into().map_err(|_| ZkError::HeaderTruncated)?))
}

/// Read raw header from section
fn read_header(section: &[u8]) -> Result<RawHeader, ZkError> {
    if section.len() < size_of::<RawHeader>() {
        return Err(ZkError::SectionTooSmall);
    }
    Ok(RawHeader {
        program_hash_off: read_u32(section, 0)?,
        capsule_commitment_off: read_u32(section, 4)?,
        public_inputs_off: read_u32(section, 8)?,
        proof_off: read_u32(section, 12)?,
        end_off: read_u32(section, 16)?,
    })
}

/// Extract slice from section with bounds checking
fn slice<'a>(section: &'a [u8], start: usize, end: usize) -> Result<&'a [u8], ZkError> {
    if start > end || end > section.len() {
        return Err(ZkError::OffsetRange);
    }
    Ok(&section[start..end])
}

/// Parse ZK proof from ELF section
pub fn parse_section(section: &[u8], manifest: Option<&[u8]>) -> Result<ZkProof, ZkError> {
    let hdr = read_header(section)?;
    let end = hdr.end_off as usize;
    if end > section.len() {
        return Err(ZkError::OffsetRange);
    }

    let ph_off = hdr.program_hash_off as usize;
    let cc_off = hdr.capsule_commitment_off as usize;
    let pi_off = hdr.public_inputs_off as usize;
    let proof_off = hdr.proof_off as usize;

    // Validate all offsets
    if ph_off + 32 > end || cc_off + 32 > end || pi_off > proof_off || proof_off > end {
        return Err(ZkError::OffsetRange);
    }

    // Extract program hash
    let mut program_hash = [0u8; 32];
    program_hash.copy_from_slice(slice(section, ph_off, ph_off + 32)?);

    // Extract capsule commitment
    let mut capsule_commitment = [0u8; 32];
    capsule_commitment.copy_from_slice(slice(section, cc_off, cc_off + 32)?);

    // Extract public inputs and proof blob
    let public_inputs = slice(section, pi_off, proof_off)?.to_vec();
    let proof_blob = slice(section, proof_off, end)?.to_vec();

    Ok(ZkProof {
        program_hash,
        capsule_commitment,
        public_inputs,
        proof_blob,
        manifest: manifest.map(|m| m.to_vec()),
    })
}

/// Validate section structure without parsing
pub fn validate_section(section: &[u8]) -> Result<(), ZkError> {
    let hdr = read_header(section)?;
    let end = hdr.end_off as usize;
    if end > section.len() {
        return Err(ZkError::OffsetRange);
    }

    let ph_off = hdr.program_hash_off as usize;
    let cc_off = hdr.capsule_commitment_off as usize;
    let pi_off = hdr.public_inputs_off as usize;
    let proof_off = hdr.proof_off as usize;

    if ph_off + 32 > end || cc_off + 32 > end || pi_off > proof_off || proof_off > end {
        return Err(ZkError::OffsetRange);
    }

    Ok(())
}
