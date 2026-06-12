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

pub fn verifier_hash(receipt_id: &[u8; 32], evidence: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new_derive_key("NONOS:NOX:ZK:VERIFIER:v1");
    h.update(receipt_id);
    h.update(evidence);
    h.update(b"PASS");
    *h.finalize().as_bytes()
}
