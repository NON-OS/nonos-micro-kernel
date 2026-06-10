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

use crate::policy_tree::POLICY_EPOCH;

pub fn circuit_id(circuit_name: &str, vk: &[u8; 32], tsha: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key("NONOS:NOX:ZK:CIRCUIT:v1");
    h.update(circuit_name.as_bytes());
    h.update(vk);
    h.update(tsha);
    h.update(&POLICY_EPOCH.to_be_bytes());
    *h.finalize().as_bytes()
}

pub fn receipt_id(
    circuit: &[u8; 32],
    evidence: &[u8; 32],
    contributor: &str,
    round: u32,
) -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key("NONOS:NOX:ZK:RECEIPT:v1");
    h.update(circuit);
    h.update(evidence);
    h.update(contributor.to_ascii_lowercase().as_bytes());
    h.update(&round.to_be_bytes());
    *h.finalize().as_bytes()
}

pub fn work_receipt_id(
    circuit: &[u8; 32],
    evidence: &[u8; 32],
    contributor: &str,
    kind: &str,
    epoch: u64,
) -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key("NONOS:NOX:ZK:RECEIPT:v1");
    h.update(circuit);
    h.update(evidence);
    h.update(contributor.to_ascii_lowercase().as_bytes());
    h.update(kind.as_bytes());
    h.update(&epoch.to_be_bytes());
    *h.finalize().as_bytes()
}
