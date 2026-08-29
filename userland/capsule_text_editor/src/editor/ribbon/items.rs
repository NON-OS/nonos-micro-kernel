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

//! The ribbon's data tables. A pill's row index selects the value it applies,
//! so order is load-bearing. Only values the document model can actually carry
//! appear here; the paragraph icons have no model behind them and stay dimmed.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq)]
pub(in crate::editor) enum RibbonItem {
    Pill(usize),
    Toggle(usize),
    Icon(usize),
}

pub(in crate::editor) const HEADINGS: [(&str, u8); 7] = [
    ("Normal text", 0),
    ("Heading 1", 1),
    ("Heading 2", 2),
    ("Heading 3", 3),
    ("Heading 4", 4),
    ("Heading 5", 5),
    ("Heading 6", 6),
];

pub(in crate::editor) const FONTS: [&str; 2] = ["Inter", "JetBrains Mono"];
pub(in crate::editor) const SIZES: [u32; 10] = [10, 11, 12, 14, 16, 18, 21, 26, 34, 48];
pub(super) const TOGGLES: [&str; 5] = ["B", "I", "U", "S", "A"];
pub(in crate::editor) const TOGGLE_LIVE: [bool; 5] = [true, true, true, true, true];
pub(super) const ICON_COUNT: usize = 6;

pub(super) fn pill_labels(pill: usize) -> Vec<String> {
    match pill {
        0 => HEADINGS.iter().map(|(name, _)| name.to_string()).collect(),
        1 => FONTS.iter().map(|name| name.to_string()).collect(),
        _ => SIZES.iter().map(|px| format!("{}", px)).collect(),
    }
}
