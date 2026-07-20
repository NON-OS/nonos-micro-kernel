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

use super::{paint_icons as ic, ui};
use crate::wallet::state::{State, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELDED};
use crate::wallet::theme::{ACCENT, FG, LINE2, MUTED};

pub const HDR_Y: u32 = 47;
pub const HDR_H: u32 = 30;
pub const ICON_W: u32 = 34;
pub const MAIN_W: u32 = 44;
pub const THEME_BTN_X: u32 = 1042;
pub const CMD_BTN_X: u32 = 1084;
pub const MSG_BTN_X: u32 = 1126;
pub const LOCK_BTN_X: u32 = 1168;
pub const MAIN_BTN_X: u32 = 1210;

pub fn paint_topbar(state: &State, fb: &mut PaintBuffer) {
    let title = match state.view {
        VIEW_RECEIVE => "Receive funds",
        VIEW_SEND => "Compose transfer",
        VIEW_PROOF => "Transaction proofs",
        VIEW_SHIELDED => "Shielded balances",
        VIEW_NOX => "NOX revenue & staking",
        _ => "Account overview",
    };
    let _ = fb.text_ttf(226, 46, title, FG(), 20.0);

    let mut c = box_edge(fb, THEME_BTN_X, ICON_W, false);
    ic::theme(fb, THEME_BTN_X, HDR_Y, state.light_mode, c);
    c = box_edge(fb, CMD_BTN_X, ICON_W, state.panel == 1);
    ic::cmd(fb, CMD_BTN_X, HDR_Y, c);
    c = box_edge(fb, MSG_BTN_X, ICON_W, state.panel == 2);
    ic::bell(fb, MSG_BTN_X, HDR_Y, c, ACCENT());
    c = box_edge(fb, LOCK_BTN_X, ICON_W, false);
    ic::lock(fb, LOCK_BTN_X, HDR_Y, c);
    c = box_edge(fb, MAIN_BTN_X, MAIN_W, state.panel == 3);
    ic::account(fb, MAIN_BTN_X, HDR_Y, c);
}

fn box_edge(fb: &mut PaintBuffer, x: u32, w: u32, open: bool) -> u32 {
    let (edge, tint) = if open { (ACCENT(), ACCENT()) } else { (LINE2(), MUTED()) };
    ui::edge(fb, x, HDR_Y, w, HDR_H, edge);
    tint
}
