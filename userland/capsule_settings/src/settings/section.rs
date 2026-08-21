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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Section {
    General,
    Network,
    Wifi,
    Security,
    Appearance,
    Privacy,
    Sound,
    Storage,
    Updates,
    Developer,
}

pub const SECTIONS: [Section; 10] = [
    Section::General,
    Section::Network,
    Section::Wifi,
    Section::Security,
    Section::Appearance,
    Section::Privacy,
    Section::Sound,
    Section::Storage,
    Section::Updates,
    Section::Developer,
];

pub const SECTION_COUNT: usize = SECTIONS.len();

impl Section {
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(i: usize) -> Section {
        SECTIONS[core::cmp::min(i, SECTION_COUNT - 1)]
    }
}
