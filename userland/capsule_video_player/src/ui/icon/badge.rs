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

use crate::ui::sprite::{cache, Glyph};
use crate::ui::theme;
use nonos_app_skeleton::paint::PaintBuffer;

pub fn play_disc(fb: &mut PaintBuffer, cx: u32, cy: u32, r: u32) {
    let d = r * 2;
    cache::draw(
        fb,
        cx.saturating_sub(r + 2),
        cy.saturating_sub(r + 2),
        d + 4,
        0x33000000,
        Glyph::Disc,
    );
    cache::draw(fb, cx.saturating_sub(r), cy.saturating_sub(r), d, theme::ACCENT, Glyph::Disc);
    let g = r;
    cache::draw(
        fb,
        cx.saturating_sub(g / 2),
        cy.saturating_sub(g / 2),
        g,
        theme::APP_BG,
        Glyph::Play,
    );
}

pub fn play_mark(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    cache::draw(fb, x, y, s, argb, Glyph::Play);
}
