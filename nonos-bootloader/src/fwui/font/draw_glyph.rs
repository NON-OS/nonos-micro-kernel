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

use super::blend::blend;
use crate::display::gop::{get_pixel, put_pixel};
use crate::fwui::metrics::raster_height;
use noto_sans_mono_bitmap::{get_raster, FontWeight};

pub fn draw_glyph(x: u32, y: u32, ch: char, fg: u32) {
    let h = raster_height();
    let raster = match get_raster(ch, FontWeight::Regular, h) {
        Some(r) => r,
        None => match get_raster('?', FontWeight::Regular, h) {
            Some(r) => r,
            None => return,
        },
    };
    for (ry, row) in raster.raster().iter().enumerate() {
        for (rx, &cov) in row.iter().enumerate() {
            if cov == 0 {
                continue;
            }
            let (px, py) = (x + rx as u32, y + ry as u32);
            put_pixel(px, py, blend(get_pixel(px, py), fg, cov));
        }
    }
}
