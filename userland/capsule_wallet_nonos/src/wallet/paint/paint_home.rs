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

use super::quick_icon::Icon;
use super::ui;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, DIM, FG, INK};

pub fn paint_home(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    let lw = 600u32;
    let rx = cx + lw + 16;
    super::paint_account_card::paint_account_card(state, fb, cx, 146, lw);
    super::paint_network_card::paint_network_card(state, fb, rx, 146, cw - lw - 16);

    let _ = fb.text_ttf(cx as i32, 366, "QUICK ACTIONS", DIM(), 12.1);
    let qw = (cw - 48) / 4;
    // Real symbols rather than the ASCII stand-ins these used to carry. The
    // bundled face has them, and an arrow reads as an arrow at a glance where
    // a caret reads as a typo.
    quick(fb, cx, 386, qw, Icon::Send, "Send");
    quick(fb, cx + qw + 16, 386, qw, Icon::Receive, "Receive");
    quick(fb, cx + 2 * (qw + 16), 386, qw, Icon::Stake, "Stake");
    quick(fb, cx + 3 * (qw + 16), 386, qw, Icon::Swap, "Swap");

    super::paint_home_activity::paint_home_activity(state, fb, cx, cw);
}

fn quick(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, kind: Icon, label: &str) {
    ui::card(fb, x, y, w, 82);
    let ix = x + w / 2 - 17;
    fb.fill_rect(ix, y + 16, 34, 34, ACCENT());
    super::quick_icon::icon(fb, ix, y + 16, kind, INK());
    let tw = fb.measure_ttf(label, 16.1).max(0) as u32;
    let _ =
        fb.text_ttf((x + w / 2).saturating_sub(tw / 2) as i32, (y + 56) as i32, label, FG(), 16.1);
}
