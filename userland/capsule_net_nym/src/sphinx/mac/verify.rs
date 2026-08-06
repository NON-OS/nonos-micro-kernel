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

use super::compute::compute_mac;
use crate::crypto::types::CryptoError;
use crate::sphinx::constants::{HEADER_INTEGRITY_MAC_SIZE, INTEGRITY_MAC_KEY_SIZE};

/// Recompute and compare without an early exit. A comparison that returned on
/// the first differing byte would leak, through timing, how much of a forged
/// MAC was right, which is enough to build the rest of it a byte at a time.
pub fn verify_mac(
    key: &[u8; INTEGRITY_MAC_KEY_SIZE],
    data: &[u8],
    expected: &[u8; HEADER_INTEGRITY_MAC_SIZE],
) -> Result<bool, CryptoError> {
    let got = compute_mac(key, data)?;
    let mut diff = 0u8;
    for (a, b) in got.iter().zip(expected.iter()) {
        diff |= a ^ b;
    }
    Ok(diff == 0)
}
