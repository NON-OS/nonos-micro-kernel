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

use nonos_attestation_circuit::compute_capsule_commitment;

pub fn commitment(
    capsule_hash: &[u8; 32],
    policy_root: &[u8; 32],
    epoch: u64,
    caps: u64,
) -> [u8; 32] {
    let mut seed = Vec::with_capacity(80);
    seed.extend_from_slice(capsule_hash);
    seed.extend_from_slice(policy_root);
    seed.extend_from_slice(&epoch.to_be_bytes());
    seed.extend_from_slice(&caps.to_be_bytes());
    compute_capsule_commitment(&seed)
}
