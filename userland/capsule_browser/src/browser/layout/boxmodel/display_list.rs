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
use alloc::vec::Vec;

pub enum Content {
    None,
    Text {
        text: String,
        color: u32,
        px: f32,
        bold: bool,
        mono: bool,
        underline: bool,
        font: u32,
        spacing: f32,
    },
    Image {
        src: String,
        alt: String,
        fit: crate::browser::css::ObjectFit,
    },
}

// One painted rectangle in absolute page coordinates. Border widths run
// top, right, bottom, left; clip is [x0, y0, x1, y1] in page coordinates.
pub struct Fragment {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub bg: u32,
    pub border: [u32; 4],
    pub border_color: u32,
    pub href: Option<String>,
    pub content: Content,
    pub z: i32,
    pub clip: Option<[i32; 4]>,
    // Painted without the scroll offset when true (position:fixed).
    pub fixed: bool,
    // background-image url to fetch and paint behind the box content.
    pub bg_image: Option<alloc::string::String>,
    pub bg_size: crate::browser::css::BgSize,
    pub bg_repeat: bool,
    // drop shadow painted behind the box.
    pub shadow: Option<crate::browser::css::Shadow>,
    pub radius: u32,
    // DOM node behind this rect (0 = anonymous), for event hit-testing.
    pub node: usize,
}

pub type DisplayList = Vec<Fragment>;

pub struct BoxDocument {
    pub frags: DisplayList,
    pub content_h: u32,
}

impl BoxDocument {
    // Fragments paint front to back, so the last hit wins.
    pub fn link_at(&self, x: i32, y: i32) -> Option<&str> {
        for f in self.frags.iter().rev() {
            if let Some(href) = f.href.as_deref() {
                if x >= f.x && x < f.x + f.w && y >= f.y && y < f.y + f.h {
                    return Some(href);
                }
            }
        }
        None
    }

    // Topmost DOM node under the point, for event dispatch.
    pub fn hit_node(&self, x: i32, y: i32) -> Option<usize> {
        for f in self.frags.iter().rev() {
            if f.node != 0 && x >= f.x && x < f.x + f.w && y >= f.y && y < f.y + f.h {
                return Some(f.node);
            }
        }
        None
    }
}
