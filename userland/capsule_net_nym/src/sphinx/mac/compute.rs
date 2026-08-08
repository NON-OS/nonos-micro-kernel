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

use crate::crypto::kdf::hmac_sha256;
use crate::crypto::types::CryptoError;
use crate::sphinx::constants::{HEADER_INTEGRITY_MAC_SIZE, INTEGRITY_MAC_KEY_SIZE};

/// HMAC-SHA256 over the encrypted routing info, truncated to the first 16
/// bytes. Truncation is the specification's, not a shortcut.
pub fn compute_mac(
    key: &[u8; INTEGRITY_MAC_KEY_SIZE],
    data: &[u8],
) -> Result<[u8; HEADER_INTEGRITY_MAC_SIZE], CryptoError> {
    let mut full = [0u8; 32];
    hmac_sha256(key, data, &mut full)?;
    let mut mac = [0u8; HEADER_INTEGRITY_MAC_SIZE];
    mac.copy_from_slice(&full[..HEADER_INTEGRITY_MAC_SIZE]);
    Ok(mac)
}
