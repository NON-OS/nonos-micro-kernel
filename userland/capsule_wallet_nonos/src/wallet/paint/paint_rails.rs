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
use crate::wallet::theme::{ACCENT_DIM, FG, MUTED};

pub fn paint_rails(state: &State, fb: &mut PaintBuffer) {
    fb.text(52, 230, b"Rails", MUTED);
    for i in 0..state.rail_count {
        let rail = state.rails[i];
        let y = 258 + (i as u32) * 28;
        let c = super::rail_color::rail_color(rail.status);
        fb.fill_rect(52, y, 204, 22, ACCENT_DIM);
        fb.fill_rect(52, y, 4, 22, c);
        fb.text(68, y + 5, &rail.symbol[..rail.symbol_len as usize], FG);
        fb.text(148, y + 5, super::rail_text::rail_text(rail.status), c);
    }
}
