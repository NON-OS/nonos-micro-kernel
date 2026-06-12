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

use super::keccak::keccak256;

pub fn claim_leaf(
    contributor: &[u8; 20],
    receipt_id: &[u8; 32],
    circuit_id: &[u8; 32],
    amount: &[u8; 32],
    epoch: u64,
    pool_id: &[u8; 32],
) -> [u8; 32] {
    let mut encoded = [0u8; 192];
    encoded[12..32].copy_from_slice(contributor);
    encoded[32..64].copy_from_slice(receipt_id);
    encoded[64..96].copy_from_slice(circuit_id);
    encoded[96..128].copy_from_slice(amount);
    encoded[152..160].copy_from_slice(&epoch.to_be_bytes());
    encoded[160..192].copy_from_slice(pool_id);
    keccak256(&keccak256(&encoded))
}
