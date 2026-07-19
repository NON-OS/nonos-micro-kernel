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

use crate::wallet::state::{State, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELDED};
use crate::wallet::theme::{ACCENT, FG, LINE2, MUTED};

pub fn paint_topbar(state: &State, fb: &mut PaintBuffer) {
    let title = match state.view {
        VIEW_RECEIVE => "Receive funds",
        VIEW_SEND => "Compose transfer",
        VIEW_PROOF => "Transaction proofs",
        VIEW_SHIELDED => "Shielded balances",
        VIEW_NOX => "NOX revenue & staking",
        _ => "Account overview",
    };
    let _ = fb.text_ttf(226, 46, title, FG, 20.0);

    let mut x = fb.width.saturating_sub(26);
    x = btn(fb, x, "Main", true);
    x = btn(fb, x - 9, "LOCK", false);
    x = btn(fb, x - 9, "MSG 3", false);
    let _ = btn(fb, x - 9, "CMD_K", false);
}

fn btn(fb: &mut PaintBuffer, right: u32, label: &str, drop: bool) -> u32 {
    let tw = fb.measure_ttf_mono(label, 11.5).max(0) as u32;
    let extra = if drop { 30 } else { 22 };
    let w = tw + extra;
    let x = right.saturating_sub(w);
    super::ui::edge(fb, x, 48, w, 28, LINE2);
    if drop {
        fb.fill_rect(x + 10, 58, 12, 12, ACCENT);
        let _ = fb.text_ttf_mono((x + 27) as i32, 55, label, MUTED, 11.5);
    } else {
        let _ = fb.text_ttf_mono((x + 11) as i32, 55, label, MUTED, 11.5);
    }
    x
}
