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

use crate::pm::state::State;
use crate::pm::theme::{CARD_BG, CARD_BORDER, MUTED};

use super::super::chrome::Rect;
use super::super::metrics::{BODY_PX, PANEL_RADIUS};
use super::super::text;

// Placeholder pane. Phase 3 replaces this body with the real screen; the empty
// card keeps the chrome verifiable on its own terms until then.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    fb.fill_round(r.x, r.y, r.w, r.h, PANEL_RADIUS, CARD_BG);
    fb.stroke_round(r.x, r.y, r.w, r.h, PANEL_RADIUS, 1, CARD_BORDER);
    let label = state.screen.nav_label();
    let top = text::centred_top(r.y, r.h, BODY_PX);
    let x = r.x + r.w.saturating_sub(text::width(fb, label, BODY_PX)) / 2;
    text::left(fb, x, top, label, MUTED, BODY_PX);
}
