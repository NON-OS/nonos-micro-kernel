// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::display::constants::COLOR_SUCCESS;
use crate::display::font::draw_string;
use crate::display::gop::{get_dimensions, is_initialized};

pub fn show_handoff_message() {
    if !is_initialized() {
        return;
    }
    let (_, h) = get_dimensions();
    let y = h - 40;
    draw_string(40, y, b"Handoff to kernel...", COLOR_SUCCESS);
}
