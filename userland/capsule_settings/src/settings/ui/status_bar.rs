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

use crate::settings::state::{State, StatusKind};

use super::bytes::as_str;
use super::metrics::{BODY_PX, PANE_PAD_X, SIDEBAR_W};
use super::text;
use super::theme::{CARD_BORDER, OK, SIDEBAR_BG, WARN};

const BAR_H: u32 = 34;

/// The strip only exists when it has something to report, so a healthy panel
/// looks like the design and a failing one still says why.
pub fn status_h(state: &State) -> u32 {
    if state.policy_ready && state.status.kind == StatusKind::Idle {
        0
    } else {
        BAR_H
    }
}

pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let h = status_h(state);
    if h == 0 {
        return;
    }
    let y = state.win_h.saturating_sub(h);
    fb.fill_rect(SIDEBAR_W, y, state.win_w.saturating_sub(SIDEBAR_W), h, SIDEBAR_BG);
    fb.fill_rect(SIDEBAR_W, y, state.win_w.saturating_sub(SIDEBAR_W), 1, CARD_BORDER);
    let (fg, msg) = message(state);
    let top = text::centred_top(y, h, BODY_PX);
    text::left(fb, SIDEBAR_W + PANE_PAD_X, top, msg, fg, BODY_PX);
}

fn message(state: &State) -> (u32, &str) {
    if !state.policy_ready {
        return (WARN, "Policy service unavailable - values shown are not stored");
    }
    let fg = match state.status.kind {
        StatusKind::Error => WARN,
        _ => OK,
    };
    (fg, as_str(state.status.as_slice()))
}
