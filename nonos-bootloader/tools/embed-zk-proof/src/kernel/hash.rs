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

pub fn compute_kernel_hash(kernel_bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(kernel_bytes).as_bytes()
}

pub fn compute_capsule_commitment(kernel_hash: &[u8; 32], program_hash: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key("NONOS:CAPSULE:COMMITMENT:v1");
    hasher.update(kernel_hash);
    hasher.update(program_hash);
    hasher.update(&0u64.to_be_bytes());
    *hasher.finalize().as_bytes()
}
