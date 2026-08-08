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

use crate::crypto::CryptoResult;

pub fn generate_random_bytes(buffer: &mut [u8]) -> CryptoResult<()> {
    for chunk in buffer.chunks_mut(8) {
        let random_u64 = generate_secure_u64()?;
        let bytes = random_u64.to_le_bytes();
        let copy_len = core::cmp::min(chunk.len(), 8);
        chunk[..copy_len].copy_from_slice(&bytes[..copy_len]);
    }
    Ok(())
}

fn generate_secure_u64() -> CryptoResult<u64> {
    if let Some(value) = crate::arch::cpu_random::random_u64() {
        return Ok(value);
    }
    if let Some(value) = crate::arch::cpu_random::entropy_u64() {
        return Ok(value);
    }
    if let Ok(buf) = try_virtio_rng() {
        return Ok(u64::from_le_bytes(buf));
    }
    Err(crate::crypto::CryptoError::InsufficientEntropy)
}

fn try_virtio_rng() -> Result<[u8; 8], ()> {
    let mut buf = [0u8; 8];
    crate::drivers::virtio_rng::fill_random(&mut buf).map_err(|_| ())?;
    Ok(buf)
}
