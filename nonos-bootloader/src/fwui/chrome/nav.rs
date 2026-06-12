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
use crate::fwui::metrics::{advance, glyph_h};
use crate::fwui::state::Section;
use crate::fwui::text::text;
use crate::fwui::theme;

pub fn nav(lay: &Layout, active: Section) {
    let n = &lay.nav;
    fill_rect(n.x, n.y, n.w, n.h + glyph_h(), theme::BG);
    let adv = advance();
    let mut x = n.x;
    for sec in Section::ALL {
        let sel = sec == active;
        let label = sec.nav_label();
        text(x, n.y, label, if sel { theme::ACCENT } else { theme::DIM });
        if sel {
            hline(x, n.y + glyph_h() + 5, label.len() as u32 * adv, theme::ACCENT);
        }
        x += (label.len() as u32 + 3) * adv;
    }
    hline(n.x, n.bottom() + glyph_h() / 2, n.w, theme::FRAME);
}
