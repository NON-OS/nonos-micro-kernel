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

use super::prefs_geom as geom;
use crate::app::prefs::{SECTIONS, SECTION_SPAN, TOGGLES};
use crate::app::state::VideoApp;
use crate::ui::layout::Rect;
use crate::ui::paint::{rrect, shape};
use crate::ui::text::BODY_PX;
use crate::ui::theme;
use crate::ui::widget::button::{paint_button, Tone};
use crate::ui::widget::empty::paint_inline;
use crate::ui::widget::section::paint_titled_card;
use crate::ui::widget::toggle::paint_row;

fn paint_rail(fb: &mut PaintBuffer, body: Rect, active: usize) {
    for (i, label) in SECTIONS.iter().enumerate() {
        let r = geom::section_row(body, i);
        let ink = if i == active {
            rrect::fill_round(fb, r.x, r.y, r.w, r.h, 8, theme::SELECT);
            theme::ACCENT
        } else {
            theme::TEXT_DIM
        };
        fb.text_ttf((r.x + 14) as i32, (r.y + 9) as i32, label, ink, BODY_PX);
    }
    let rail = geom::rail(body);
    shape::vline(fb, rail.x + rail.w, rail.y, rail.h, theme::BORDER);
}

fn paint_actions(fb: &mut PaintBuffer, body: Rect) {
    paint_button(fb, geom::reset_button(body), "Reset to Defaults", Tone::Quiet);
    paint_button(fb, geom::cancel_button(body), "Cancel", Tone::Ghost);
    paint_button(fb, geom::save_button(body), "Save Changes", Tone::Primary);
}

pub fn paint(fb: &mut PaintBuffer, app: &VideoApp, body: Rect) {
    let active = app.prefs.section;
    paint_rail(fb, body, active);
    paint_titled_card(fb, geom::card(body), SECTIONS[active]);
    let (base, len) = SECTION_SPAN[active];
    if len == 0 {
        paint_inline(fb, geom::content(body), "No options in this section yet");
    }
    for slot in 0..len {
        let index = base + slot;
        paint_row(fb, geom::toggle_row(body, slot), TOGGLES[index], "", app.prefs.get(index));
    }
    paint_actions(fb, body);
}
