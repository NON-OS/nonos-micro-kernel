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

#[cfg(feature = "zk-groth16")]
use super::entries::core_circuits;
#[cfg(feature = "zk-groth16")]
use super::entries_fingerprint::entries_with_fingerprint;
#[cfg(feature = "zk-groth16")]
use super::lookup_error::LookupError;
#[cfg(feature = "zk-groth16")]
use super::vk_fingerprint::verify_vk_fingerprint;
#[cfg(feature = "zk-groth16")]
use crate::zk::verify::ct_eq32;

#[cfg(feature = "zk-groth16")]
pub fn lookup_verified(program_hash: &[u8; 32]) -> Result<&'static [u8], LookupError> {
    for entry in core_circuits() {
        if ct_eq32(&entry.program_hash, program_hash) && !entry.vk_bytes.is_empty() {
            return Ok(entry.vk_bytes);
        }
    }
    for (h, vk, fp) in entries_with_fingerprint() {
        if ct_eq32(&h, program_hash) {
            return if verify_vk_fingerprint(vk, &fp) {
                Ok(vk)
            } else {
                Err(LookupError::FingerprintMismatch)
            };
        }
    }
    Err(LookupError::NotFound)
}

#[cfg(feature = "zk-groth16")]
pub fn lookup(program_hash: &[u8; 32]) -> Option<&'static [u8]> {
    lookup_verified(program_hash).ok()
}
