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

use crate::display::constants::COLOR_TEXT_DIM;
use crate::display::font::draw_string;
use crate::display::gop::is_initialized;

pub fn init_boot_screen() {
    if !is_initialized() {
        return;
    }
    super::vignette::draw_vignette();
    super::wordmark::draw_wordmark(40, 24);
    draw_string(40, 62, b"secure attestation boot", COLOR_TEXT_DIM);
}

pub fn reset_animation() {}

pub fn tick_animation() {}
