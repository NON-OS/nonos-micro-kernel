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

use crate::protocol::STATUS_OK;

use super::store::advance;

pub fn tick(payload: &[u8], reply: &mut [u8]) -> (u16, usize) {
    let delta = if payload.len() >= 8 {
        let mut a = [0u8; 8];
        a.copy_from_slice(&payload[0..8]);
        u64::from_le_bytes(a)
    } else {
        0
    };
    let new = advance(delta);
    if reply.len() >= 8 {
        reply[0..8].copy_from_slice(&new.to_le_bytes());
        (STATUS_OK, 8)
    } else {
        (STATUS_OK, 0)
    }
}
