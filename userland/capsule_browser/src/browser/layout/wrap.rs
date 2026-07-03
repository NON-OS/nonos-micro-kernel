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

pub const MARGIN: u32 = 16;
const FG: u32 = 0xFFE6_EDF3;
const LINK: u32 = 0xFF4C_9AFF;

pub struct Cursor {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub advance: u32,
}

pub fn word(
    cur: &mut Cursor,
    w: &str,
    href: Option<String>,
    scale: u32,
    bold: bool,
    req_color: u32,
    bg: u32,
) -> (Span, bool) {
    let em = super::px_for(scale);
    let text_w = nonos_app_skeleton::measure_ttf(w, em).max(0) as u32;
    let space_w = nonos_app_skeleton::measure_ttf(" ", em).max(1) as u32;
    let px = text_w + space_w + if bold { 1 } else { 0 };
    let wrapped = if cur.x + px > cur.width.saturating_sub(MARGIN) {
        cur.y += scale * 8 + 12;
        cur.x = MARGIN;
        true
    } else {
        false
    };
    let color = if href.is_some() {
        LINK
    } else if req_color != 0 {
        req_color
    } else {
        FG
    };
    let span = Span { x: cur.x, w: px, text: String::from(w), color, bg, href, scale, bold };
    cur.x += px;
    (span, wrapped)
}
