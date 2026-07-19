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

use crate::wallet::theme::{ACCENT, FG, PANEL_2};

pub fn paint_button(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, text: &[u8]) {
    fb.fill_rect(x, y, w, 42, PANEL_2());
    fb.fill_rect(x, y + 40, w, 2, ACCENT());
    fb.text(x + 22, y + 14, text, FG());
}
