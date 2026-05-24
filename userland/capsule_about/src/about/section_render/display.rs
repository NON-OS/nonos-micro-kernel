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

use super::row;
use crate::about::data::display::primary_dimensions;
use crate::about::format::u64_decimal;
use crate::about::state::VISIBLE_BODY_LINES;

pub const LINE_COUNT: u32 = 4;

pub fn render(scroll: u32, top: u32, fb: &mut PaintBuffer) {
    let dims = primary_dimensions();
    let mut buf_w = [0u8; 20];
    let mut buf_h = [0u8; 20];
    let (w_str, h_str): (&[u8], &[u8]) = match dims {
        Some((w, h)) => (u64_decimal(w as u64, &mut buf_w), u64_decimal(h as u64, &mut buf_h)),
        None => (b"unavailable", b"unavailable"),
    };
    let rows: [(&[u8], &[u8]); 4] = [
        (b"Backend", b"compositor + driver.virtio_gpu"),
        (b"Format", b"ARGB8888"),
        (b"Width (px)", w_str),
        (b"Height (px)", h_str),
    ];
    let end = (scroll + VISIBLE_BODY_LINES).min(LINE_COUNT);
    for visible in 0..(end - scroll) {
        let (label, value) = rows[(scroll + visible) as usize];
        row::pair(label, value, row::line_y(visible, top), fb);
    }
}
