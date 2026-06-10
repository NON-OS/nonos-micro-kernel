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

mod derive;
mod entries;
mod entries_fingerprint;
mod lookup_circuit;
mod lookup_error;
mod lookup_verified;
mod parse;
mod parse_entry;
mod parse_header;
mod parse_verify;
mod program_hash;
mod store;
mod types_category;
mod types_entry;
mod types_permission;
mod types_section;
mod types_verify;
mod validate;
mod vk_data;
mod vk_fingerprint;

pub use derive::{derive_circuit_key, verify_circuit_key_derivation};
pub use parse::parse_circuit_section;
pub use store::DynamicCircuitStore;
pub use types_category::CircuitCategory;
pub use types_entry::{CircuitEntry, DynamicCircuitEntry};
pub use types_permission::CircuitPermission;
pub use types_section::{CircuitSectionEntry, CircuitSectionHeader, CIRCUIT_SECTION_MAGIC};

#[cfg(feature = "zk-groth16")]
pub use entries::core_circuits;
#[cfg(feature = "zk-groth16")]
pub use entries_fingerprint::entries_with_fingerprint;
#[cfg(feature = "zk-groth16")]
pub use lookup_circuit::{circuits_with_permission, has_permission, lookup_circuit};
#[cfg(feature = "zk-groth16")]
pub use lookup_error::LookupError;
#[cfg(feature = "zk-groth16")]
pub use lookup_verified::{lookup, lookup_verified};
#[cfg(feature = "zk-groth16")]
pub use program_hash::{
    PROGRAM_HASH_ATTESTATION_PROGRAM, PROGRAM_HASH_BOOT_AUTHORITY, PROGRAM_HASH_RECOVERY_KEY,
};
#[cfg(feature = "zk-groth16")]
pub use program_hash::{PROGRAM_HASH_UPDATE_AUTHORITY, ZK_REGISTRY_FINGERPRINT};
#[cfg(feature = "zk-groth16")]
pub use program_hash::{ZK_BUILD_TIMESTAMP, ZK_FROM_CEREMONY, ZK_REGISTRY_VERSION};
#[cfg(feature = "zk-groth16")]
pub use validate::{get_registry_info, validate_registry_metadata};
#[cfg(feature = "zk-groth16")]
pub use vk_data::{
    vk_attestation_program, vk_boot_authority, vk_recovery_key, vk_update_authority,
};
#[cfg(feature = "zk-groth16")]
pub use vk_fingerprint::VK_FINGERPRINT_UPDATE_AUTHORITY;
#[cfg(feature = "zk-groth16")]
pub use vk_fingerprint::{compute_vk_fingerprint, verify_vk_fingerprint};
#[cfg(feature = "zk-groth16")]
pub use vk_fingerprint::{VK_FINGERPRINT_BOOT_AUTHORITY, VK_FINGERPRINT_RECOVERY_KEY};
