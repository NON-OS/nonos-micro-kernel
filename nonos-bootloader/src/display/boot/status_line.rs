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

use super::chips::draw_chip;
use crate::display::gop::is_initialized;

pub fn draw_status_line(secure_boot: bool, measured: bool, attested: bool) {
    if !is_initialized() {
        return;
    }
    let mut x = 40;
    x = draw_chip(x, 96, b"SecureBoot", secure_boot as u8 * 2);
    x = draw_chip(x, 96, b"Measured", measured as u8 * 2);
    let _ = draw_chip(x, 96, b"Attested", attested as u8 * 2);
}
