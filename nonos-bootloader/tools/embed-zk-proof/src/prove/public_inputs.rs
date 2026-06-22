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

pub fn public_inputs(
    kernel_hash: &[u8; 32],
    boot_nonce: &[u8; 32],
    machine_id: &[u8; 32],
    timestamp: u64,
) -> [u8; 104] {
    let mut out = [0u8; 104];
    out[0..32].copy_from_slice(kernel_hash);
    out[32..64].copy_from_slice(boot_nonce);
    out[64..72].copy_from_slice(&timestamp.to_le_bytes());
    out[72..104].copy_from_slice(machine_id);
    out
}
