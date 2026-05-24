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

use crate::protocol::{Request, E_INVAL, E_RANGE, MAX_ENTRY_BYTES};
use crate::server::respond;
use crate::state::Clipboard;

pub fn run(clipboard: &mut Clipboard, req: &Request, payload: &[u8], out: &mut [u8], now_ms: u64) -> usize {
    if payload.len() < 4 {
        return respond::status(out, req, E_INVAL);
    }
    let content_type = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let data = &payload[4..];
    if data.len() > MAX_ENTRY_BYTES {
        return respond::status(out, req, E_RANGE);
    }
    clipboard.copy(content_type, data, now_ms);
    respond::status(out, req, 0)
}
