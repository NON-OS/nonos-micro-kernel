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
use crate::fwui::metrics::{advance, glyph_h, line, pad};
use crate::fwui::text::text;
use crate::fwui::theme;

pub fn row(x: u32, y: u32, w: u32, label: &[u8], value: &[u8], vcolor: u32, selected: bool) {
    let h = line();
    if selected {
        fill_rect(x, y, w, h, theme::SEL_BG);
        fill_rect(x, y, 2, h, theme::ACCENT);
    }
    let ty = y + (h - glyph_h()) / 2;
    text(x + pad(), ty, label, if selected { theme::TEXT } else { theme::DIM });
    let vw = value.len() as u32 * advance();
    text(x + w - pad() - vw, ty, value, if selected { theme::ACCENT } else { vcolor });
    hline(x, y + h - 1, w, theme::ROWLINE);
}
