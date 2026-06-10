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
const DS_VK_FINGERPRINT: &str = "NONOS:VK:FINGERPRINT:v1";

#[cfg(feature = "zk-groth16")]
pub fn compute_vk_fingerprint(vk_bytes: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key(DS_VK_FINGERPRINT);
    h.update(vk_bytes);
    *h.finalize().as_bytes()
}

#[cfg(feature = "zk-groth16")]
pub fn verify_vk_fingerprint(vk_bytes: &[u8], expected: &[u8; 32]) -> bool {
    let computed = compute_vk_fingerprint(vk_bytes);
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= computed[i] ^ expected[i];
    }
    diff == 0
}

#[cfg(feature = "zk-groth16")]
pub use super::program_hash::VK_FINGERPRINT_RECOVERY_KEY;
#[cfg(feature = "zk-groth16")]
pub use super::program_hash::{VK_FINGERPRINT_BOOT_AUTHORITY, VK_FINGERPRINT_UPDATE_AUTHORITY};
