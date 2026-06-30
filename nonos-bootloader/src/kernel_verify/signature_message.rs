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

pub fn signed_kernel_message(kernel_hash: &[u8; 32], rollback_index: u32) -> [u8; 36] {
    let mut signed_message = [0u8; 36];
    signed_message[..32].copy_from_slice(kernel_hash);
    signed_message[32..].copy_from_slice(&rollback_index.to_le_bytes());
    signed_message
}
