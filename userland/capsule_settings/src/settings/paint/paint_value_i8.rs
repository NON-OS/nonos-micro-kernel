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

use crate::settings::theme::{VALUE_FG, VALUE_FG_FALSE};

use super::fmt_signed::fmt_signed;
use super::layout::VALUE_LEFT;

pub fn paint_value_i8(fb: &mut PaintBuffer, y: u32, value: Option<i8>) {
    fb.text(VALUE_LEFT, y, b"<", VALUE_FG);
    let mut buf = [0u8; 5];
    let n = match value {
        Some(v) => fmt_signed(v as i32, &mut buf),
        None => {
            buf[0] = b'.';
            buf[1] = b'.';
            buf[2] = b'.';
            3
        }
    };
    fb.text(VALUE_LEFT + 18, y, &buf[..n], VALUE_FG);
    fb.text(VALUE_LEFT + 64, y, b">", VALUE_FG);
    fb.text(VALUE_LEFT + 86, y, b"hours UTC", VALUE_FG_FALSE);
}
