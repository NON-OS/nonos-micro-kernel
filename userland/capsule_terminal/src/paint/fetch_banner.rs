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

//! The NONOS block banner, the same art the build prints. JetBrains Mono has
//! the block and box-drawing glyphs, so it renders crisply through the mono
//! TrueType path.

use nonos_app_skeleton::PaintBuffer;

use crate::term::theme::types::Theme;

const BANNER: [&str; 6] = [
    "███╗   ██╗ ██████╗ ███╗   ██╗ ██████╗ ███████╗",
    "████╗  ██║██╔═══██╗████╗  ██║██╔═══██╗██╔════╝",
    "██╔██╗ ██║██║   ██║██╔██╗ ██║██║   ██║███████╗",
    "██║╚██╗██║██║   ██║██║╚██╗██║██║   ██║╚════██║",
    "██║ ╚████║╚██████╔╝██║ ╚████║╚██████╔╝███████║",
    "╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝",
];

const BANNER_PX: f32 = 13.0;
const BANNER_ROW: i32 = 15;

// Draw the banner at (x, y). Returns the y just below it.
pub fn draw_banner(fb: &mut PaintBuffer, x: i32, y: i32, t: &Theme) -> i32 {
    let mut yy = y;
    for line in BANNER {
        let _ = fb.text_ttf_mono(x, yy, line, t.accent, BANNER_PX);
        yy += BANNER_ROW;
    }
    yy
}
