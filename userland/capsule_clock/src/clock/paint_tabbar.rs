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

use crate::clock::tabs::{self, Tab, BAR_H, BAR_TOP};
use crate::clock::theme;

pub fn paint(tab: Tab, fb: &mut PaintBuffer, width: u32) {
    let w = width / 4;
    for (i, t) in tabs::all().iter().enumerate() {
        let x = i as u32 * w;
        let active = *t == tab;
        let bg = if active { theme::ACCENT } else { theme::BG };
        fb.fill_rect(x, BAR_TOP as u32, w, BAR_H as u32, bg);
        let fg = if active { theme::BG } else { theme::DIM };
        fb.text(x + 10, (BAR_TOP + 10) as u32, tabs::label(*t), fg);
    }
}
