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

extern crate alloc;

use alloc::vec::Vec;

use super::decode_validation;
use super::error::DecodeError;
use super::reader::Reader;
use super::strings::{bounded_bytes, bounded_count, bounded_string};
use crate::limits::{
    MAX_ARCHES, MAX_CAPABILITIES, MAX_NAME, MAX_PUBLISHER, MAX_SIGNATURE, MAX_SUPPORTED_ARCH_LEN,
    MAX_URL,
};
use crate::types::CapsuleRelease;

pub(super) fn read(r: &mut Reader<'_>) -> Result<CapsuleRelease, DecodeError> {
    let release_id = bounded_string(r, MAX_NAME)?;
    let manifest_hash = r.fixed::<32>()?;
    let package_hash = r.fixed::<32>()?;
    let package_url = bounded_string(r, MAX_URL)?;
    let publisher_signature = bounded_bytes(r, MAX_SIGNATURE)?;

    let arch_count = bounded_count(r, MAX_ARCHES)?;
    let mut supported_arches: Vec<alloc::string::String> = Vec::with_capacity(arch_count as usize);
    for _ in 0..arch_count {
        supported_arches.push(bounded_string(r, MAX_SUPPORTED_ARCH_LEN)?);
    }

    let kernel_abi_min = r.u32()?;

    let cap_count = bounded_count(r, MAX_CAPABILITIES)?;
    let mut required_capabilities: Vec<alloc::string::String> =
        Vec::with_capacity(cap_count as usize);
    for _ in 0..cap_count {
        required_capabilities.push(bounded_string(r, MAX_PUBLISHER)?);
    }

    let validation = decode_validation::read(r)?;

    Ok(CapsuleRelease {
        release_id,
        manifest_hash,
        package_hash,
        package_url,
        publisher_signature,
        supported_arches,
        kernel_abi_min,
        required_capabilities,
        validation,
    })
}
