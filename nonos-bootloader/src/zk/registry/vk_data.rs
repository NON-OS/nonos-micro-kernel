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
use super::program_hash::{
    VK_ALL_BYTES, VK_ATTESTATION_PROGRAM_LEN, VK_ATTESTATION_PROGRAM_OFFSET,
};
#[cfg(feature = "zk-groth16")]
use super::program_hash::{VK_BOOT_AUTHORITY_LEN, VK_BOOT_AUTHORITY_OFFSET};
#[cfg(feature = "zk-groth16")]
use super::program_hash::{VK_RECOVERY_KEY_LEN, VK_RECOVERY_KEY_OFFSET};
#[cfg(feature = "zk-groth16")]
use super::program_hash::{VK_UPDATE_AUTHORITY_LEN, VK_UPDATE_AUTHORITY_OFFSET};

#[cfg(feature = "zk-groth16")]
pub fn vk_attestation_program() -> &'static [u8] {
    &VK_ALL_BYTES
        [VK_ATTESTATION_PROGRAM_OFFSET..VK_ATTESTATION_PROGRAM_OFFSET + VK_ATTESTATION_PROGRAM_LEN]
}

#[cfg(feature = "zk-groth16")]
pub fn vk_boot_authority() -> &'static [u8] {
    &VK_ALL_BYTES[VK_BOOT_AUTHORITY_OFFSET..VK_BOOT_AUTHORITY_OFFSET + VK_BOOT_AUTHORITY_LEN]
}

#[cfg(feature = "zk-groth16")]
pub fn vk_update_authority() -> &'static [u8] {
    &VK_ALL_BYTES[VK_UPDATE_AUTHORITY_OFFSET..VK_UPDATE_AUTHORITY_OFFSET + VK_UPDATE_AUTHORITY_LEN]
}

#[cfg(feature = "zk-groth16")]
pub fn vk_recovery_key() -> &'static [u8] {
    &VK_ALL_BYTES[VK_RECOVERY_KEY_OFFSET..VK_RECOVERY_KEY_OFFSET + VK_RECOVERY_KEY_LEN]
}
