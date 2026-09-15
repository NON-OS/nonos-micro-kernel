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

extern crate alloc;

use alloc::string::{String, ToString};

/// One laid-out line on a stacked screen: either a section heading, or a row
/// that navigates to `path`. Each screen builds these once, its painter draws
/// exactly them, and `screen_hit` tests against them, so a click on any of these
/// surfaces lands on the row that was actually drawn.
pub struct Line {
    pub y: u32,
    pub h: u32,
    pub head: Option<&'static str>,
    pub path: String,
    pub meta: String,
    pub dir: bool,
}

impl Line {
    pub fn head(y: u32, h: u32, text: &'static str) -> Line {
        Line { y, h, head: Some(text), path: String::new(), meta: String::new(), dir: false }
    }

    pub fn row(y: u32, h: u32, path: &str, meta: String, dir: bool) -> Line {
        Line { y, h, head: None, path: path.to_string(), meta, dir }
    }
}
