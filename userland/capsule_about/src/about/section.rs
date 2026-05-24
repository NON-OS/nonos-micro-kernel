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

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Section {
    Identity,
    Authority,
    Display,
    Uptime,
    License,
}

pub const SECTIONS: [Section; 5] = [
    Section::Identity,
    Section::Authority,
    Section::Display,
    Section::Uptime,
    Section::License,
];

impl Section {
    pub fn title(self) -> &'static [u8] {
        match self {
            Section::Identity => b"Identity",
            Section::Authority => b"Authority",
            Section::Display => b"Display",
            Section::Uptime => b"Uptime",
            Section::License => b"License",
        }
    }
    pub fn index(self) -> usize {
        match self {
            Section::Identity => 0,
            Section::Authority => 1,
            Section::Display => 2,
            Section::Uptime => 3,
            Section::License => 4,
        }
    }
}
