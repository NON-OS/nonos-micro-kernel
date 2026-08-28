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
use core::fmt::Write;

use crate::doc::style::{Family, RunStyle};

pub fn escape(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
}

pub fn half_points(size_px: f32) -> u32 {
    let v = size_px * 2.0 + 0.5;
    if v < 2.0 {
        2
    } else {
        v as u32
    }
}

pub fn rpr(s: &RunStyle, out: &mut String) {
    out.push_str("<w:rPr>");
    if s.family == Family::Mono {
        out.push_str("<w:rFonts w:ascii=\"Courier New\" w:hAnsi=\"Courier New\" w:cs=\"Courier New\"/>");
    }
    if s.bold {
        out.push_str("<w:b/>");
    }
    if s.italic {
        out.push_str("<w:i/>");
    }
    if s.strike {
        out.push_str("<w:strike/>");
    }
    let _ = write!(out, "<w:sz w:val=\"{}\"/>", half_points(s.size_px));
    if s.underline {
        out.push_str("<w:u w:val=\"single\"/>");
    }
    out.push_str("</w:rPr>");
}
