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

use crate::wallet::theme::{FG, MUTED, PANEL};

pub fn paint_rail_card(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    symbol: &[u8],
    label: &[u8],
    color: u32,
) {
    fb.fill_rect(x, y, w, 54, PANEL);
    fb.fill_rect(x, y, 5, 54, color);
    fb.text(x + 20, y + 12, symbol, FG);
    fb.text(x + 76, y + 12, label, MUTED);
}
