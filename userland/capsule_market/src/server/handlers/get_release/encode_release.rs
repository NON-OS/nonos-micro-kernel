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
use nonos_marketplace_abi::CapsuleRelease;

use super::write_lp_string::write_lp_string;

pub(super) fn encode_release(rel: &CapsuleRelease) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    write_lp_string(&mut out, &rel.release_id);
    out.extend_from_slice(&rel.manifest_hash);
    out.extend_from_slice(&rel.package_hash);
    write_lp_string(&mut out, &rel.package_url);
    out.extend_from_slice(&(rel.publisher_signature.len() as u32).to_le_bytes());
    out.extend_from_slice(&rel.publisher_signature);
    out.extend_from_slice(&(rel.supported_arches.len() as u32).to_le_bytes());
    for arch in &rel.supported_arches {
        write_lp_string(&mut out, arch);
    }
    out.extend_from_slice(&rel.kernel_abi_min.to_le_bytes());
    out.extend_from_slice(&(rel.required_capabilities.len() as u32).to_le_bytes());
    for cap in &rel.required_capabilities {
        write_lp_string(&mut out, cap);
    }
    out.push(rel.validation.status as u8);
    write_lp_string(&mut out, &rel.validation.note);
    write_lp_string(&mut out, &rel.validation.validator_id);
    out.extend_from_slice(&rel.validation.validated_at_ms.to_le_bytes());
    out
}
