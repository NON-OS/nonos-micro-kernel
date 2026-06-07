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

use crate::display::constants::{COLOR_SUCCESS, COLOR_WARNING};
use crate::display::font::draw_string;
use crate::display::gop::is_initialized;

pub fn draw_status_line(secure_boot: bool, measured: bool, attested: bool) {
    if !is_initialized() {
        return;
    }
    draw_flag(40, 122, b"SecureBoot", secure_boot);
    draw_flag(200, 122, b"Measured", measured);
    draw_flag(360, 122, b"Attested", attested);
}

fn draw_flag(x: u32, y: u32, label: &[u8], ok: bool) {
    let color = if ok { COLOR_SUCCESS } else { COLOR_WARNING };
    draw_string(x, y, label, color);
}
