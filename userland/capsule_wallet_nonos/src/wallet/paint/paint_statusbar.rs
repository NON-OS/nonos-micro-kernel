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
use crate::wallet::theme::{FG, MUTED, PANEL_2};

pub fn paint_statusbar(state: &State, fb: &mut PaintBuffer) {
    let y = fb.height.saturating_sub(54);
    fb.fill_rect(304, y, fb.width.saturating_sub(304), 54, PANEL_2);
    fb.text(336, y + 20, b"Status", MUTED);
    fb.text(424, y + 20, state.status, FG);
}
