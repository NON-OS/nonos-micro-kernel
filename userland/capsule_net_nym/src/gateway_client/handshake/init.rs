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

use super::sizes::{EPHEMERAL_BYTES, IDENTITY_BYTES, INIT_BYTES, SALT_BYTES};

/// First message: who we are, a throwaway key, and the salt that binds the
/// derived key to this exchange.
pub fn init_message(
    identity: &[u8; IDENTITY_BYTES],
    ephemeral: &[u8; EPHEMERAL_BYTES],
    salt: &[u8; SALT_BYTES],
) -> [u8; INIT_BYTES] {
    let mut out = [0u8; INIT_BYTES];
    out[..IDENTITY_BYTES].copy_from_slice(identity);
    out[IDENTITY_BYTES..IDENTITY_BYTES + EPHEMERAL_BYTES].copy_from_slice(ephemeral);
    out[IDENTITY_BYTES + EPHEMERAL_BYTES..].copy_from_slice(salt);
    out
}
