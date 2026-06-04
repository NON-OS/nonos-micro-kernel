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

use crate::command::output::Output;

use super::emit_line::emit_line;
use super::read_u32::read_u32;

pub(super) fn render_entries(out: &mut Output<'_>, body: &[u8], count: u32) {
    let mut cursor = 0;
    for _ in 0..count {
        let Some(listing_len) = read_u32(body, cursor).map(|n| n as usize) else { return };
        cursor += 4;
        if cursor + listing_len > body.len() {
            return;
        }
        let listing = &body[cursor..cursor + listing_len];
        cursor += listing_len;
        if cursor + 32 > body.len() {
            return;
        }
        cursor += 32;
        let Some(name_len) = read_u32(body, cursor).map(|n| n as usize) else { return };
        cursor += 4;
        if cursor + name_len + 1 > body.len() {
            return;
        }
        let name = &body[cursor..cursor + name_len];
        cursor += name_len;
        let ready = body[cursor] != 0;
        cursor += 1;
        emit_line(out, listing, name, ready);
    }
}
