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
pub enum Mode {
    Basic,
    Scientific,
    Programmer,
    Convert,
    History,
}

pub const MODES: [Mode; 5] = [
    Mode::Basic,
    Mode::Scientific,
    Mode::Programmer,
    Mode::Convert,
    Mode::History,
];

impl Mode {
    pub fn index(self) -> usize {
        match self {
            Mode::Basic => 0,
            Mode::Scientific => 1,
            Mode::Programmer => 2,
            Mode::Convert => 3,
            Mode::History => 4,
        }
    }
    pub fn from_index(i: usize) -> Option<Mode> {
        MODES.get(i).copied()
    }
    pub fn label(self) -> &'static str {
        match self {
            Mode::Basic => "Basic",
            Mode::Scientific => "Scientific",
            Mode::Programmer => "Programmer",
            Mode::Convert => "Convert",
            Mode::History => "History",
        }
    }
}
