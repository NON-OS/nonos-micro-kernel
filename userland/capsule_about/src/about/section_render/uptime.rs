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
use crate::about::data::uptime::{read_millis, split_dhms};
use crate::about::format::u64_decimal;

pub const LINE_COUNT: u32 = 5;

pub fn render(scroll: u32, visible: u32, top: u32, fb: &mut PaintBuffer) {
    let mut buf_ms = [0u8; 20];
    let mut buf_d = [0u8; 20];
    let mut buf_h = [0u8; 20];
    let mut buf_m = [0u8; 20];
    let mut buf_s = [0u8; 20];
    let ms = read_millis();
    let (d, h, m, s) = match ms {
        Some(v) => split_dhms(v),
        None => (0, 0, 0, 0),
    };
    let ms_str: &[u8] = match ms {
        Some(v) => u64_decimal(v, &mut buf_ms),
        None => b"unavailable",
    };
    let rows: [(&[u8], &[u8]); 5] = [
        (b"Wall ms", ms_str),
        (b"Days", u64_decimal(d, &mut buf_d)),
        (b"Hours", u64_decimal(h, &mut buf_h)),
        (b"Minutes", u64_decimal(m, &mut buf_m)),
        (b"Seconds", u64_decimal(s, &mut buf_s)),
    ];
    let end = scroll.saturating_add(visible).min(LINE_COUNT);
    let visible_count = end.saturating_sub(scroll);
    for offset in 0..visible_count {
        let (label, value) = rows[(scroll + offset) as usize];
        row::pair(label, value, row::line_y(offset, top), fb);
    }
}
