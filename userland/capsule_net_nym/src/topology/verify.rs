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

use crate::crypto::ed25519_verify::verify as ed25519_verify;

use super::layout;
use super::types::TopologyError;
use crate::state;

pub fn check(body: &[u8]) -> Result<(), TopologyError> {
    let msg = layout::signed_message(body);
    let issuer = &body[32..64];
    match state::trusted_authority(issuer) {
        Some(true) => {}
        Some(false) => return Err(TopologyError::UntrustedAuthority),
        None => return Err(TopologyError::NoAuthority),
    }
    let sig = &body[64..128];
    if ed25519_verify(issuer, sig, &msg) != 0 {
        return Err(TopologyError::BadSignature);
    }
    Ok(())
}
