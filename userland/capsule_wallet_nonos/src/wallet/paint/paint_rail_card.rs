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

use crate::wallet::theme::{FG, MUTED};

pub fn paint_rail_card(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    _w: u32,
    symbol: &[u8],
    label: &[u8],
    color: u32,
) {
    fb.fill_rect(x, y, 3, 44, color);
    let s = core::str::from_utf8(symbol).unwrap_or("");
    let l = core::str::from_utf8(label).unwrap_or("");
    let _ = fb.text_ttf((x + 16) as i32, (y + 2) as i32, s, FG(), 16.0);
    let _ = fb.text_ttf((x + 16) as i32, (y + 24) as i32, l, MUTED(), 12.0);
}
