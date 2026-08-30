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

//! The floating panels the menu bar opens. Both follow the dropdown shape: a
//! measured box over the shell whose row index doubles as the action selector.

use alloc::vec::Vec;

use super::specials::SPECIALS;

#[derive(Clone, Copy, PartialEq)]
pub(in crate::editor) enum Panel {
    WordCount,
    Special,
}

pub(in crate::editor) const PANEL_PAD: u32 = 14;
pub(in crate::editor) const PANEL_SLACK: u32 = 44;
pub(in crate::editor) const PANEL_R: u32 = 10;

pub(in crate::editor) fn panel_title(panel: Panel) -> &'static str {
    match panel {
        Panel::WordCount => "Word Count",
        Panel::Special => "Special Character",
    }
}

pub(in crate::editor) fn special_labels() -> Vec<&'static str> {
    SPECIALS.iter().map(|(label, _)| *label).collect()
}
