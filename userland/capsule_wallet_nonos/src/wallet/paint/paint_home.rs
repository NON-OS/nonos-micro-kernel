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

use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, CYAN, MUTED, WARN};

pub fn paint_home(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width.saturating_sub(368);
    let left_w = (w / 2).saturating_sub(20);
    let right_x = 336 + w / 2 + 8;
    super::panel::panel(fb, 336, 128, left_w, 232);
    super::panel::panel(fb, right_x, 128, left_w, 232);
    super::panel::panel(fb, 336, 388, w.saturating_sub(32), fb.height.saturating_sub(470));
    super::paint_account_card::paint_account_card(state, fb);
    super::paint_network_card::paint_network_card(state, fb, right_x + 32);
    fb.text(368, 420, b"Enabled rails", MUTED);
    let gap = 32;
    let rail_w = core::cmp::max(128, w.saturating_sub(128) / 3);
    super::paint_rail_card::paint_rail_card(fb, 368, 456, rail_w, b"ETH", b"native", CYAN);
    super::paint_rail_card::paint_rail_card(
        fb,
        368 + rail_w + gap,
        456,
        rail_w,
        b"NOX",
        b"ERC-20",
        ACCENT,
    );
    super::paint_rail_card::paint_rail_card(
        fb,
        368 + (rail_w + gap) * 2,
        456,
        rail_w,
        b"PR",
        b"reserved",
        WARN,
    );
    super::paint_home_security::paint_home_security(fb, w);
}
