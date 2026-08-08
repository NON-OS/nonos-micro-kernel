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

use super::keystream::pseudorandom_bytes;
use super::step::filler_step;
use crate::sphinx::constants::MAX_PATH_LENGTH;
use crate::sphinx::keys::ExpandedSharedSecret;
use alloc::vec::Vec;

/// The padding a header carries so its length never reveals how many hops are
/// left. Built from every hop's key except the last one's.
pub fn build_filler(secrets: &[ExpandedSharedSecret]) -> Option<Vec<u8>> {
    if secrets.len() > MAX_PATH_LENGTH {
        return None;
    }
    let mut acc = Vec::new();
    for (idx, secret) in secrets.iter().enumerate() {
        let prng = pseudorandom_bytes(&secret.stream_cipher_key());
        acc = filler_step(acc, idx + 1, &prng)?;
    }
    Some(acc)
}
