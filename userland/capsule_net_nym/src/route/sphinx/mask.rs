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

extern crate alloc;

use alloc::vec::Vec;

use crate::crypto::hkdf_sha256;
use crate::packet::PacketError;

pub fn apply(
    region: &mut [u8],
    seed: &[u8; 32],
    idx: u8,
    key: &[u8; 32],
) -> Result<(), PacketError> {
    let mut info = Vec::with_capacity(57);
    info.extend_from_slice(b"NONOS-NYM-SPHINX-MASK-v1");
    info.extend_from_slice(seed);
    info.push(idx);
    let mut mask = Vec::new();
    mask.resize(region.len(), 0);
    hkdf_sha256(seed, key, &info, &mut mask).map_err(|_| PacketError::Crypto)?;
    for i in 0..region.len() {
        region[i] ^= mask[i];
    }
    mask.fill(0);
    Ok(())
}
