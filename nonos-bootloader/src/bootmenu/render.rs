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

use super::footer::draw_footer;
use super::header::draw_header;
use super::list::draw_list;
use crate::display::fx::fill_atmosphere;
use crate::display::gop::get_dimensions;
use crate::security::SecurityContext;

pub(super) fn render(sel: usize, remaining_s: u32, _sec: &SecurityContext) {
    let (w, h) = get_dimensions();
    fill_atmosphere();

    // Vertically center the title + list + status as one block.
    let title_y = h.saturating_sub(360) / 2 + 24;
    let list_top = title_y + 116;
    let status_y = list_top + 6 * 50 + 30;

    draw_header(w, title_y);
    draw_list(w, list_top, sel);
    draw_footer(w, status_y, remaining_s);
}
