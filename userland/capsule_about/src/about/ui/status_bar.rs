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

use crate::about::state::State;
use crate::about::theme::{ACCENT, MUTED, RULE, STATUS_BG};

use super::metrics::{BODY_PX, STATUS_GROUP_GAP, STATUS_H, STATUS_PAD_X};
use super::text;

// The strip under every screen. Left is what the window is licensed under,
// right is how to leave it: the two facts that are true on all five sections
// and therefore belong to the frame rather than to any one of them.
pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let y = state.fb_h.saturating_sub(STATUS_H);
    fb.fill_rect(0, y, state.fb_w, STATUS_H, STATUS_BG);
    fb.fill_rect(0, y, state.fb_w, 1, RULE);
    let top = text::top_of(y as i32, STATUS_H, BODY_PX);
    let advance = text::line(fb, STATUS_PAD_X, top, b"License ", MUTED, BODY_PX);
    text::line(fb, advance.max(0) as u32, top, b"AGPL-3.0", ACCENT, BODY_PX);
    let right = state.fb_w.saturating_sub(STATUS_PAD_X);
    text::right(fb, right, top, b"Esc closes", MUTED, BODY_PX);
    let hint = b"Tab cycles sections";
    let hint_right = right.saturating_sub(text::width(fb, b"Esc closes", BODY_PX))
        .saturating_sub(STATUS_GROUP_GAP);
    if hint_right > STATUS_PAD_X * 6 {
        text::right(fb, hint_right, top, hint, MUTED, BODY_PX);
    }
}
