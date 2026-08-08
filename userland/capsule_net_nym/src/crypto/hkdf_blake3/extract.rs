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

use super::super::types::CryptoError;
use super::hmac::hmac_blake3;

/// HKDF-Extract: the salt is the key and the input keying material the
/// message, which is the opposite of what the names suggest.
pub fn extract(salt: &[u8], ikm: &[u8], prk: &mut [u8; 32]) -> Result<(), CryptoError> {
    hmac_blake3(salt, ikm, prk)
}
