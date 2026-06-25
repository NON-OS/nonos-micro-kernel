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

use alloc::string::String;

use crate::browser::layout::doc::Span;

pub const LINE_H: u32 = 20;
pub const MARGIN: u32 = 16;
const FG: u32 = 0xFFE6_EDF3;
const LINK: u32 = 0xFF4C_9AFF;

pub struct Cursor {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub advance: u32,
}

pub fn word(cur: &mut Cursor, w: &str, href: Option<String>) -> (Span, bool) {
    let px = w.len() as u32 * cur.advance + cur.advance;
    let wrapped = if cur.x + px > cur.width.saturating_sub(MARGIN) {
        cur.y += LINE_H;
        cur.x = MARGIN;
        true
    } else {
        false
    };
    let color = if href.is_some() { LINK } else { FG };
    let span = Span {
        x: cur.x,
        w: px,
        text: String::from(w),
        color,
        href,
        image_src: None,
    };
    cur.x += px;
    (span, wrapped)
}
