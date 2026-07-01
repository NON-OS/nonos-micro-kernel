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

use super::types::{Frame, HEADER, MAGIC, VERSION};

pub fn decode(bytes: &[u8]) -> Option<Frame<'_>> {
    if bytes.len() < HEADER {
        return None;
    }
    let magic = u32::from_le_bytes(bytes[0..4].try_into().ok()?);
    if magic != MAGIC || bytes[4] != VERSION || bytes[5] != 1 {
        return None;
    }
    let len = u16::from_le_bytes(bytes[14..16].try_into().ok()?) as usize;
    if HEADER + len > bytes.len() {
        return None;
    }
    let ip = [bytes[8], bytes[9], bytes[10], bytes[11]];
    let port = u16::from_le_bytes(bytes[12..14].try_into().ok()?);
    Some(Frame { ip, port, body: &bytes[HEADER..HEADER + len] })
}
