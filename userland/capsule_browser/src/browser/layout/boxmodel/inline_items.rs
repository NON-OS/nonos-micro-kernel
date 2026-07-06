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

// One atom of an inline formatting context, pre-measured at collect time.
pub(super) enum InlineItem {
    Word {
        text: String,
        px: u32,
        color: u32,
        bg: u32,
        bold: bool,
        mono: bool,
        underline: bool,
        font: u32,
        href: Option<String>,
        adv: i32,
        space: i32,
        line_h: i32,
        node: usize,
    },
    Image {
        src: String,
        alt: String,
        w: i32,
        h: i32,
        href: Option<String>,
        node: usize,
        fit: crate::browser::css::ObjectFit,
    },
    Break,
}

impl InlineItem {
    pub(super) fn advance_w(&self) -> i32 {
        match self {
            InlineItem::Word { adv, .. } => *adv,
            InlineItem::Image { w, .. } => *w,
            InlineItem::Break => 0,
        }
    }

    pub(super) fn item_h(&self) -> i32 {
        match self {
            InlineItem::Word { line_h, .. } => *line_h,
            InlineItem::Image { h, .. } => *h,
            InlineItem::Break => 0,
        }
    }

    pub(super) fn space_w(&self) -> i32 {
        match self {
            InlineItem::Word { space, .. } => *space,
            _ => 0,
        }
    }
}
