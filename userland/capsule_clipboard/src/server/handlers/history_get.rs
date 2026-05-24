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

use crate::protocol::{Request, E_INVAL, E_RANGE, HDR_LEN, STATUS_LEN};
use crate::server::respond;
use crate::state::Clipboard;

pub fn run(clipboard: &mut Clipboard, req: &Request, payload: &[u8], out: &mut [u8], now_ms: u64) -> usize {
    if payload.len() < 4 {
        return respond::status(out, req, E_INVAL);
    }
    let index = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
    clipboard.touch(now_ms);
    let entry = match clipboard.get_by_index(index) {
        Some(e) => e,
        None => return respond::status(out, req, E_RANGE),
    };
    let data = &entry.data;
    let dst = HDR_LEN + STATUS_LEN;
    if dst + 8 + data.len() > out.len() {
        return respond::status(out, req, E_RANGE);
    }
    out[dst..dst + 4].copy_from_slice(&entry.content_type.to_le_bytes());
    out[dst + 4..dst + 8].copy_from_slice(&(data.len() as u32).to_le_bytes());
    out[dst + 8..dst + 8 + data.len()].copy_from_slice(data);
    respond::with_payload(out, req, 0, 8 + data.len())
}
