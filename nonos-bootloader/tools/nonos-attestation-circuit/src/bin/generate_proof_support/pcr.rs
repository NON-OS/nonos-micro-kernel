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

use nonos_attestation_circuit::PCR_PREIMAGE_LEN;

pub fn pcr(seed: &str, capsule_hash: &[u8; 32], commitment: &[u8; 32]) -> [u8; PCR_PREIMAGE_LEN] {
    let mut out = [0u8; PCR_PREIMAGE_LEN];
    let mut hasher = blake3::Hasher::new();
    hasher.update(seed.as_bytes());
    hasher.update(capsule_hash);
    hasher.update(commitment);
    let hash = hasher.finalize();
    out[..32].copy_from_slice(hash.as_bytes());
    out[32..].copy_from_slice(hash.as_bytes());
    out
}
