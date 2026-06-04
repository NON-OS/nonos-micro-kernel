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

use alloc::vec::Vec;

pub fn build_request(out: &mut Vec<u8>, op: u16, request_id: u32, payload: &[u8]) {
    out.clear();
    out.extend_from_slice(&super::NVGP_MAGIC.to_le_bytes());
    out.extend_from_slice(&super::NVGP_VERSION.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&request_id.to_le_bytes());
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
}
