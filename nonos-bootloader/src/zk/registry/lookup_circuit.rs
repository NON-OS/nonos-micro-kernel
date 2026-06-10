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
use super::types_category::CircuitCategory;
#[cfg(feature = "zk-groth16")]
use super::types_entry::CircuitEntry;
#[cfg(feature = "zk-groth16")]
use super::types_permission::CircuitPermission;
#[cfg(feature = "zk-groth16")]
use crate::zk::verify::ct_eq32;

#[cfg(feature = "zk-groth16")]
pub fn lookup_circuit(program_hash: &[u8; 32]) -> Option<CircuitEntry> {
    for entry in core_circuits() {
        if ct_eq32(&entry.program_hash, program_hash) {
            if entry.category == CircuitCategory::Core
                && entry.signature.is_some()
                && !entry.has_valid_signature()
            {
                return None;
            }
            return Some(entry);
        }
    }
    None
}

#[cfg(feature = "zk-groth16")]
pub fn has_permission(program_hash: &[u8; 32], permission: CircuitPermission) -> bool {
    lookup_circuit(program_hash).map_or(false, |e| (e.permissions & (permission as u32)) != 0)
}

#[cfg(feature = "zk-groth16")]
pub fn circuits_with_permission(permission: CircuitPermission) -> alloc::vec::Vec<CircuitEntry> {
    core_circuits().into_iter().filter(|e| (e.permissions & (permission as u32)) != 0).collect()
}
