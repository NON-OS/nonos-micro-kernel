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

use super::constants::CAPSULE_ZK_MAGIC;

pub fn trailer(proof: &[u8], public_inputs: &[u8], commitment: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + 4 + proof.len() + 4 + public_inputs.len() + 32);
    out.extend_from_slice(CAPSULE_ZK_MAGIC);
    out.extend_from_slice(&(proof.len() as u32).to_le_bytes());
    out.extend_from_slice(proof);
    out.extend_from_slice(&(public_inputs.len() as u32).to_le_bytes());
    out.extend_from_slice(public_inputs);
    out.extend_from_slice(commitment);
    out
}
