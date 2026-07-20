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

use crate::wallet::theme::{GREEN, LINE, WIDTH};

pub fn paint_statusline(fb: &mut PaintBuffer) {
    let y = 98u32;
    fb.fill_rect(226, y + 11, 9, 9, GREEN());
    let _ = fb.text_ttf(246, (y + 9) as i32, "keys sealed  \u{00b7}  TLS secured  \u{00b7}  route local  \u{00b7}  security STRONG", GREEN(), 12.5);
    fb.fill_rect(200, 130, WIDTH - 200, 1, LINE());
}
