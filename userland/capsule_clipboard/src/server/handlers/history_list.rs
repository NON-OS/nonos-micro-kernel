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

use crate::protocol::{Request, HDR_LEN, STATUS_LEN};
use crate::server::respond;
use crate::state::Clipboard;

pub fn run(clipboard: &Clipboard, req: &Request, out: &mut [u8]) -> usize {
    let count = clipboard.iter().count() as u32;
    let dst = HDR_LEN + STATUS_LEN;
    if dst + 4 + (count as usize) * 8 > out.len() {
        return respond::status(out, req, crate::protocol::E_RANGE);
    }
    out[dst..dst + 4].copy_from_slice(&count.to_le_bytes());
    let mut p = dst + 4;
    for entry in clipboard.iter() {
        out[p..p + 4].copy_from_slice(&entry.content_type.to_le_bytes());
        out[p + 4..p + 8].copy_from_slice(&(entry.len() as u32).to_le_bytes());
        p += 8;
    }
    respond::with_payload(out, req, 0, 4 + (count as usize) * 8)
}
