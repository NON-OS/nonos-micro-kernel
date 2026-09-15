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

use nonos_app_skeleton::PaintBuffer;

use super::entries::Entry;
use super::fmt_time::fmt_time;
use super::human_size::human_size;
use super::info_row::{kv, KV_ADV};
use super::open_with_table::handlers_for;

// The manager only ever reports what the store told it, so an unknown size or
// time renders as a dash rather than as a zero.
const UNKNOWN: &str = "--";

/// Size, time and permissions for a file, then every handler registered for its
/// extension. A file with no registered handler says so instead of showing an
/// empty list.
pub fn file_body(fb: &mut PaintBuffer, entry: &Entry, x: u32, top: u32, w: u32) -> u32 {
    let mut y = top;
    match entry.size {
        Some(size) => y += kv(fb, x, y, w, "Size", &human_size(size)),
        None => y += kv(fb, x, y, w, "Size", UNKNOWN),
    }
    let when = fmt_time(entry.mtime);
    y += kv(fb, x, y, w, "Modified", if entry.mtime == 0 { UNKNOWN } else { when.as_str() });
    let perms = if entry.writable { "read / write" } else { "read only" };
    y += kv(fb, x, y, w, "Permissions", perms);
    let handlers = handlers_for(&entry.full_path);
    if handlers.is_empty() {
        y += kv(fb, x, y, w, "Opens with", "no handler");
    }
    for (i, handler) in handlers.iter().enumerate() {
        let label = if i == 0 { "Opens with" } else { "" };
        y += kv(fb, x, y, w, label, handler);
    }
    y + KV_ADV / 2
}
