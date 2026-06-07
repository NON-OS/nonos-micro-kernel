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

use super::constants::PUBLIC_INPUT_BYTES;

pub fn public_inputs(
    capsule: &[u8; 32],
    program: &[u8; 32],
    caps: u64,
    commitment: &[u8; 32],
) -> Vec<u8> {
    let mut out = Vec::with_capacity(PUBLIC_INPUT_BYTES);
    for half in [&capsule[..16], &capsule[16..], &program[..16], &program[16..]] {
        out.extend_from_slice(&[0u8; 16]);
        out.extend_from_slice(half);
    }
    out.extend_from_slice(&[0u8; 24]);
    out.extend_from_slice(&caps.to_be_bytes());
    for half in [&commitment[..16], &commitment[16..]] {
        out.extend_from_slice(&[0u8; 16]);
        out.extend_from_slice(half);
    }
    out
}
