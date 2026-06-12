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

use crate::fwui::draw::{fill_rect, hline};
use crate::fwui::layout::Layout;
use crate::fwui::metrics::{advance, glyph_h, line};
use crate::fwui::section::Row;
use crate::fwui::state::Section;
use crate::fwui::text::text;
use crate::fwui::theme;
use crate::fwui::widget::row;

pub fn center(lay: &Layout, section: Section, rows: &[Row], cursor: usize) {
    let c = &lay.content;
    fill_rect(c.x, c.y, c.w, c.h, theme::BG);
    let title = section.title();
    text(c.x, c.y, title, theme::ACCENT);
    hline(
        c.x + title.len() as u32 * advance() + advance(),
        c.y + glyph_h() / 2,
        c.w.saturating_sub(title.len() as u32 * advance() + advance()),
        theme::FRAME,
    );
    let mut y = c.y + line() * 2;
    for (i, r) in rows.iter().enumerate() {
        row(c.x, y, c.w, r.label, r.value.as_bytes(), r.vcolor, i == cursor);
        y += line();
    }
}
