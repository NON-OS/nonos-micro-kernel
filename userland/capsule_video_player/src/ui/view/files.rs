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

use super::grid::paint_list;
use crate::app::state::VideoApp;
use crate::ui::icon;
use crate::ui::layout::Rect;
use crate::ui::paint::{rrect, shape};
use crate::ui::text::{center_y, BODY_PX};
use crate::ui::theme;
use crate::ui::widget::button::{paint_button, Tone};
use crate::ui::widget::empty::paint_empty;

const RAIL_W: u32 = 190;
const GAP: u32 = 20;
const ROW_H: u32 = 36;
const ACTIONS_H: u32 = 44;
const BTN_W: u32 = 130;
const BTN_H: u32 = 38;
const BTN_GAP: u32 = 10;

const LOCATIONS: [&str; 5] = ["Home", "Movies", "Series", "Downloads", "Clips"];

pub fn paint(fb: &mut PaintBuffer, app: &VideoApp, body: Rect) {
    let rail = Rect { x: body.x, y: body.y, w: RAIL_W, h: body.h };
    fb.text_ttf(rail.x as i32, rail.y as i32, "LOCATIONS", theme::LABEL, BODY_PX);
    for (i, name) in LOCATIONS.iter().enumerate() {
        let r = Rect { x: rail.x, y: rail.y + 28 + i as u32 * ROW_H, w: rail.w, h: ROW_H };
        let active = i == 0;
        let (fill, ink) =
            if active { (theme::SELECT, theme::ACCENT) } else { (0, theme::TEXT_DIM) };
        if active {
            rrect::fill_round(fb, r.x, r.y, r.w, r.h, 8, fill);
        }
        icon::nav::files(fb, r.x + 8, r.y + r.h.saturating_sub(18) / 2, 18, ink);
        fb.text_ttf((r.x + 34) as i32, center_y(r.y, r.h), name, ink, BODY_PX);
    }
    shape::vline(fb, rail.x + rail.w, rail.y, rail.h, theme::BORDER);

    let right_x = rail.x + rail.w + GAP;
    let right_w = body.w.saturating_sub(rail.w + GAP);
    let table_h = body.h.saturating_sub(ACTIONS_H);
    let right = Rect { x: right_x, y: body.y, w: right_w, h: table_h };

    if app.browse.items.is_empty() {
        paint_empty(
            fb,
            right,
            icon::nav::files,
            "This folder is empty",
            "No playable video files were found in this location",
        );
    } else {
        paint_list(fb, &app.browse, right);
    }

    let actions_y = body.y + body.h.saturating_sub(BTN_H);
    let open =
        Rect { x: right_x + right_w.saturating_sub(BTN_W), y: actions_y, w: BTN_W, h: BTN_H };
    let add = Rect { x: open.x.saturating_sub(BTN_W + BTN_GAP), y: actions_y, w: BTN_W, h: BTN_H };
    let import =
        Rect { x: add.x.saturating_sub(BTN_W + BTN_GAP), y: actions_y, w: BTN_W, h: BTN_H };
    paint_button(fb, import, "Import", Tone::Ghost);
    paint_button(fb, add, "Add to Playlist", Tone::Ghost);
    paint_button(fb, open, "Open", Tone::Primary);
}
