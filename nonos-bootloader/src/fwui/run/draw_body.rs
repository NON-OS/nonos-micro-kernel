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

use crate::fwui::chrome::statusline;
use crate::fwui::data::Sys;
use crate::fwui::layout::Layout;
use crate::fwui::render::center;
use crate::fwui::section::Row;
use crate::fwui::state::Section;

pub fn draw_body(lay: &Layout, sys: &Sys, section: Section, rows: &[Row], cursor: usize) {
    center(lay, section, rows, cursor);
    let desc = rows.get(cursor).map(|r| r.desc).unwrap_or(b"");
    statusline(lay, sys, desc);
}
