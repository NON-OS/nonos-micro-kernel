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

use crate::wallet::theme::PANEL;

pub fn panel(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    fb.fill_rect(x, y, w, h, PANEL);
    fb.fill_rect(x, y, w, 1, 0xFF2E_3A48);
    fb.fill_rect(x, y + h.saturating_sub(1), w, 1, 0xFF0A_0E13);
}
