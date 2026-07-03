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

use alloc::string::String;

use crate::browser::css::computed::Computed;
use crate::browser::css::vars::resolve;

use super::align::apply_align;
use super::border::apply_border;
use super::display::apply_display;
use super::flex::apply_flex;
use super::grid::apply_grid;
use super::list::apply_list;
use super::margin::apply_margin;
use super::padding::apply_padding;
use super::paint::apply_paint;
use super::position::apply_position;
use super::sizing::apply_sizing;
use super::text::apply_text;

// Apply one declaration to a computed style. Each domain applier claims the
// properties it owns and returns true; unknown properties fall through and
// are ignored.
pub fn apply_decl(
    c: &mut Computed,
    name: &str,
    value: &str,
    parent_fs: u32,
    vars: &[(String, String)],
) {
    // Custom property definitions are gathered globally; skip them here.
    if name.starts_with("--") {
        return;
    }
    // Substitute var() so every value parser below sees a plain literal.
    let resolved = resolve(value, vars, 0);
    let value = resolved.as_str();
    let fs = c.font_size_px;
    let _ = apply_text(c, name, value, fs, parent_fs)
        || apply_margin(c, name, value, fs)
        || apply_padding(c, name, value, fs)
        || apply_border(c, name, value, fs)
        || apply_sizing(c, name, value, fs)
        || apply_display(c, name, value)
        || apply_flex(c, name, value)
        || apply_align(c, name, value)
        || apply_grid(c, name, value, fs)
        || apply_list(c, name, value)
        || apply_position(c, name, value, fs)
        || apply_paint(c, name, value, fs);
}
