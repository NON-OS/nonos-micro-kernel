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
use nonos_toolkit::font::ttf::line_height;

use crate::settings::section::Section;
use crate::settings::section_text::{subtitle, title};

use super::icon_glyph::draw_glyph;
use super::icon_table::glyph;
use super::metrics::{HEAD_ICON, PANE_PAD_X, SUBTITLE_PX, TITLE_PX};
use super::text;
use super::theme::{ACCENT, SUBTITLE_FG, TITLE_FG};

pub fn paint(fb: &mut PaintBuffer, section: Section, top: i32) {
    let x = PANE_PAD_X;
    draw_glyph(fb, glyph(section), x, top.max(0) as u32, HEAD_ICON, ACCENT);
    let text_x = x + HEAD_ICON + 14;
    text::left(fb, text_x, top, title(section), TITLE_FG, TITLE_PX);
    let below = top + line_height(TITLE_PX) + 2;
    text::left(fb, text_x, below, subtitle(section), SUBTITLE_FG, SUBTITLE_PX);
}
