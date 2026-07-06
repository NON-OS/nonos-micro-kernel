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

use super::preview::Preview;
use super::preview_paint::VISIBLE_LINES;

pub fn info(preview: &Preview) -> alloc::string::String {
    let total = preview.lines.len();
    let first = if total == 0 { 0 } else { preview.scroll + 1 };
    let last = (preview.scroll + VISIBLE_LINES).min(total);
    let kind = if preview.binary { "binary" } else { "text" };
    let cut = if preview.truncated { " (truncated)" } else { "" };
    alloc::format!("{} bytes  {}  ln {}-{}/{}{}", preview.byte_len, kind, first, last, total, cut)
}
