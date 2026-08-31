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

use nonos_app_skeleton::paint::PaintBuffer;

use crate::ui::fit::{center_x, width};
use crate::ui::layout::Rect;
use crate::ui::paint::rrect;
use crate::ui::text::{center_y, BODY_PX};
use crate::ui::theme;

pub type Glyph = fn(&mut PaintBuffer, u32, u32, u32, u32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Primary,
    Ghost,
    Quiet,
}

fn palette(tone: Tone) -> (u32, u32, u32) {
    match tone {
        Tone::Primary => (theme::ACCENT_DIM, theme::ACCENT, theme::APP_BG),
        Tone::Ghost => (theme::PANEL, theme::BORDER, theme::TEXT),
        Tone::Quiet => (theme::PANEL_ALT, theme::BORDER_SOFT, theme::TEXT_DIM),
    }
}

pub fn paint_button(fb: &mut PaintBuffer, r: Rect, label: &str, tone: Tone) {
    let (bg, border, ink) = palette(tone);
    rrect::panel(fb, r.x, r.y, r.w, r.h, 8, bg, border);
    fb.text_ttf(center_x(label, r.x, r.w, BODY_PX), center_y(r.y, r.h), label, ink, BODY_PX);
}

pub fn paint_lead_button(fb: &mut PaintBuffer, r: Rect, glyph: Glyph, label: &str, tone: Tone) {
    let (bg, border, ink) = palette(tone);
    rrect::panel(fb, r.x, r.y, r.w, r.h, 8, bg, border);
    let icon = 16;
    let span = icon + 8 + width(label, BODY_PX);
    let start = r.x + r.w.saturating_sub(span) / 2;
    glyph(fb, start, r.y + r.h.saturating_sub(icon) / 2, icon, ink);
    fb.text_ttf((start + icon + 8) as i32, center_y(r.y, r.h), label, ink, BODY_PX);
}

pub fn paint_icon_button(fb: &mut PaintBuffer, r: Rect, glyph: Glyph, size: u32, tint: u32) {
    glyph(
        fb,
        r.x + r.w.saturating_sub(size) / 2,
        r.y + r.h.saturating_sub(size) / 2,
        size,
        tint,
    );
}
