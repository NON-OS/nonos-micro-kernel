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
use nonos_libc::crypto_random;

use crate::store::types::StoreError;

use super::constants::KEY_LEN;

pub(in crate::store) fn fresh_key() -> Result<[u8; KEY_LEN], StoreError> {
    let mut k = [0u8; KEY_LEN];
    if crypto_random(k.as_mut_ptr(), KEY_LEN) < 0 {
        return Err(StoreError::CryptoFailure);
    }
    Ok(k)
}
