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

pub fn signed_message(kernel_data: &[u8], rollback_index: u32) -> Vec<u8> {
    let mut msg = Vec::with_capacity(36);
    msg.extend_from_slice(blake3::hash(kernel_data).as_bytes());
    msg.extend_from_slice(&rollback_index.to_le_bytes());
    msg
}
