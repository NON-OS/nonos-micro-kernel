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

extern crate alloc;
use super::parse_entry::parse_single_entry;
use super::parse_header::validate_header;
use super::parse_verify::verify_section_signature;
use super::types_entry::DynamicCircuitEntry;
use alloc::vec::Vec;

pub fn parse_circuit_section(
    section: &[u8],
    verify_signature: bool,
    trusted_signers: &[[u8; 32]],
) -> Result<Vec<DynamicCircuitEntry>, &'static str> {
    let (_version, count, signature, signer) = validate_header(section)?;
    if verify_signature {
        verify_section_signature(section, &signature, &signer, trusted_signers)?;
    }
    let mut entries = Vec::with_capacity(count);
    let mut offset = 112;
    for _ in 0..count {
        entries.push(parse_single_entry(section, &mut offset)?);
    }
    Ok(entries)
}
