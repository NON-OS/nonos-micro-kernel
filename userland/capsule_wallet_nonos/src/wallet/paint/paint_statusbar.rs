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

use super::scale;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::state::State;
use crate::wallet::theme::{DIM, GREEN, LINE, MUTED, SYSBAR};

/// Space kept between the message and the chain name, so a message cut to
/// the last pixel still reads as two things rather than one.
const GAP: u32 = 24;

pub fn paint_statusbar(state: &State, fb: &mut PaintBuffer) {
    let y = fb.height.saturating_sub(30);
    fb.fill_rect(200, y, fb.width.saturating_sub(200), 30, SYSBAR());
    fb.fill_rect(200, y, fb.width.saturating_sub(200), 1, LINE());
    let sx = fb.text_ttf(226, (y + 8) as i32, "STATUS: ", MUTED(), scale::BODY);
    /*
     * The chain the wallet transacts on, not a fabricated block height. Placed
     * first because the message is cut to what is left after it.
     */
    let right = "Ethereum mainnet";
    let w = fb.measure_ttf(right, scale::BODY).max(0) as u32;
    let right_x = fb.width.saturating_sub(26 + w);
    let _ = fb.text_ttf(right_x as i32, (y + 8) as i32, right, DIM(), scale::BODY);
    let msg = core::str::from_utf8(state.status).unwrap_or("ready");
    let room = right_x.saturating_sub(sx.max(0) as u32 + GAP);
    let cut = super::status_fit::fit(fb, msg, room);
    let _ = fb.text_ttf(sx, (y + 8) as i32, cut, GREEN(), scale::BODY);
}
