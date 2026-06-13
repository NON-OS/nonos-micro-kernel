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

use crate::wallet::theme::{BG, BG_2, LINE};

pub fn paint_background(fb: &mut PaintBuffer) {
    fb.clear(BG);
    fb.fill_rect(0, 0, 304, fb.height, BG_2);
    fb.fill_rect(304, 0, 1, fb.height, LINE);
    fb.fill_rect(304, 88, fb.width.saturating_sub(304), 1, LINE);
}
