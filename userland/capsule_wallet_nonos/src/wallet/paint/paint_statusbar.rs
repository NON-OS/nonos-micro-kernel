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
use crate::wallet::theme::{DIM, GREEN, LINE, MUTED, SYSBAR};

pub fn paint_statusbar(_state: &State, fb: &mut PaintBuffer) {
    let y = fb.height.saturating_sub(30);
    fb.fill_rect(200, y, fb.width.saturating_sub(200), 30, SYSBAR());
    fb.fill_rect(200, y, fb.width.saturating_sub(200), 1, LINE());
    let sx = fb.text_ttf(226, (y + 8) as i32, "STATUS: ", MUTED(), 12.0);
    let _ = fb.text_ttf(sx, (y + 8) as i32, "wallet ready", GREEN(), 12.0);
    let right = "block 20,914,332  \u{00b7}  12:39";
    let w = fb.measure_ttf(right, 12.0).max(0) as u32;
    let _ = fb.text_ttf((fb.width - 26 - w) as i32, (y + 8) as i32, right, DIM(), 12.0);
}
