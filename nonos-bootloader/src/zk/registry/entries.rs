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
use super::program_hash::PROGRAM_HASH_RECOVERY_KEY;
#[cfg(feature = "zk-groth16")]
use super::program_hash::{
    PROGRAM_HASH_ATTESTATION_PROGRAM, PROGRAM_HASH_BOOT_AUTHORITY, PROGRAM_HASH_UPDATE_AUTHORITY,
};
#[cfg(feature = "zk-groth16")]
use super::types_category::CircuitCategory;
#[cfg(feature = "zk-groth16")]
use super::types_entry::CircuitEntry;
#[cfg(feature = "zk-groth16")]
use super::types_permission::CircuitPermission;
#[cfg(feature = "zk-groth16")]
use super::vk_data::{
    vk_attestation_program, vk_boot_authority, vk_recovery_key, vk_update_authority,
};

#[cfg(feature = "zk-groth16")]
pub fn core_circuits() -> [CircuitEntry; 4] {
    [
        CircuitEntry {
            program_hash: PROGRAM_HASH_ATTESTATION_PROGRAM,
            vk_bytes: vk_attestation_program(),
            name: "attestation-program",
            version: "1.0.0",
            permissions: CircuitPermission::BootAuthority as u32
                | CircuitPermission::Attestation as u32,
            category: CircuitCategory::Core,
            signature: None,
            signer: None,
        },
        CircuitEntry {
            program_hash: PROGRAM_HASH_BOOT_AUTHORITY,
            vk_bytes: vk_boot_authority(),
            name: "boot-authority",
            version: "1.0.0",
            permissions: CircuitPermission::BootAuthority as u32
                | CircuitPermission::Attestation as u32,
            category: CircuitCategory::Core,
            signature: None,
            signer: None,
        },
        CircuitEntry {
            program_hash: PROGRAM_HASH_UPDATE_AUTHORITY,
            vk_bytes: vk_update_authority(),
            name: "update-authority",
            version: "1.0.0",
            permissions: CircuitPermission::UpdateAuthority as u32,
            category: CircuitCategory::Core,
            signature: None,
            signer: None,
        },
        CircuitEntry {
            program_hash: PROGRAM_HASH_RECOVERY_KEY,
            vk_bytes: vk_recovery_key(),
            name: "recovery-key",
            version: "1.0.0",
            permissions: CircuitPermission::RecoveryKey as u32,
            category: CircuitCategory::Core,
            signature: None,
            signer: None,
        },
    ]
}
