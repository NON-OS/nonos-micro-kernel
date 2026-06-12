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
use super::program_hash::PROGRAM_HASH_UPDATE_AUTHORITY;
#[cfg(feature = "zk-groth16")]
use super::program_hash::{PROGRAM_HASH_BOOT_AUTHORITY, PROGRAM_HASH_RECOVERY_KEY};
#[cfg(feature = "zk-groth16")]
use super::vk_data::{vk_boot_authority, vk_recovery_key, vk_update_authority};
#[cfg(feature = "zk-groth16")]
use super::vk_fingerprint::VK_FINGERPRINT_UPDATE_AUTHORITY;
#[cfg(feature = "zk-groth16")]
use super::vk_fingerprint::{VK_FINGERPRINT_BOOT_AUTHORITY, VK_FINGERPRINT_RECOVERY_KEY};

#[cfg(feature = "zk-groth16")]
pub fn entries_with_fingerprint() -> [([u8; 32], &'static [u8], [u8; 32]); 3] {
    [
        (PROGRAM_HASH_BOOT_AUTHORITY, vk_boot_authority(), VK_FINGERPRINT_BOOT_AUTHORITY),
        (PROGRAM_HASH_UPDATE_AUTHORITY, vk_update_authority(), VK_FINGERPRINT_UPDATE_AUTHORITY),
        (PROGRAM_HASH_RECOVERY_KEY, vk_recovery_key(), VK_FINGERPRINT_RECOVERY_KEY),
    ]
}
