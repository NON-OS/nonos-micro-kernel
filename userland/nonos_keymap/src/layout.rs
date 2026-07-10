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

/// The keyboard layouts a driver can resolve. `from_index`/`next` give the
/// drivers a stable cycle order for the layout-switch hotkey, and `name`
/// is what they announce when the layout changes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Layout {
    Us,
    Uk,
    De,
    Fr,
    Es,
    It,
}

impl Layout {
    pub const COUNT: u8 = 6;

    pub fn from_index(i: u8) -> Layout {
        match i % Self::COUNT {
            0 => Layout::Us,
            1 => Layout::Uk,
            2 => Layout::De,
            3 => Layout::Fr,
            4 => Layout::Es,
            _ => Layout::It,
        }
    }

    pub fn index(self) -> u8 {
        match self {
            Layout::Us => 0,
            Layout::Uk => 1,
            Layout::De => 2,
            Layout::Fr => 3,
            Layout::Es => 4,
            Layout::It => 5,
        }
    }

    pub fn next(self) -> Layout {
        Layout::from_index(self.index().wrapping_add(1))
    }

    pub fn name(self) -> &'static [u8] {
        match self {
            Layout::Us => b"us",
            Layout::Uk => b"uk",
            Layout::De => b"de",
            Layout::Fr => b"fr",
            Layout::Es => b"es",
            Layout::It => b"it",
        }
    }
}
