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

use core::sync::atomic::{AtomicU32, Ordering};
use noto_sans_mono_bitmap::{get_raster_width, FontWeight, RasterHeight};

static SIZE: AtomicU32 = AtomicU32::new(20);

pub fn set_from_screen(h: u32) {
    let s = if h < 720 {
        16
    } else if h < 1080 {
        20
    } else if h < 1600 {
        24
    } else {
        32
    };
    SIZE.store(s, Ordering::Relaxed);
}

pub fn raster_height() -> RasterHeight {
    match SIZE.load(Ordering::Relaxed) {
        16 => RasterHeight::Size16,
        20 => RasterHeight::Size20,
        24 => RasterHeight::Size24,
        _ => RasterHeight::Size32,
    }
}

pub fn glyph_h() -> u32 {
    SIZE.load(Ordering::Relaxed)
}

pub fn advance() -> u32 {
    get_raster_width(FontWeight::Regular, raster_height()) as u32
}

pub fn line() -> u32 {
    glyph_h() + glyph_h() / 2
}

pub fn pad() -> u32 {
    advance() * 2
}

pub fn margin() -> u32 {
    glyph_h()
}
