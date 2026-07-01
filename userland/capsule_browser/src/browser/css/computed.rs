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

pub const DEFAULT_FG: u32 = 0xFFE6_EDF3;

#[derive(Clone, Copy)]
pub struct Computed {
    pub display_none: bool,
    pub color: u32,
    pub bg: u32,
    pub bold: bool,
}

impl Computed {
    pub fn root() -> Self {
        Computed { display_none: false, color: DEFAULT_FG, bg: 0, bold: false }
    }
}
