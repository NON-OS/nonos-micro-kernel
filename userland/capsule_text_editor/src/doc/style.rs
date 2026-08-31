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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Family {
    Sans,
    Mono,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RunStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub size_px: f32,
    pub family: Family,
    pub color: u32,
    pub highlight: u32,
}

impl RunStyle {
    pub fn body() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            strike: false,
            size_px: 16.0,
            family: Family::Sans,
            color: 0xFFE4ECF5,
            highlight: 0,
        }
    }

    pub fn heading(level: u8) -> Self {
        let sizes = [34.0, 26.0, 21.0, 18.0, 16.5, 15.0];
        let idx = (level.clamp(1, 6) - 1) as usize;
        Self {
            bold: true,
            size_px: sizes[idx],
            color: 0xFF17BED9,
            ..Self::body()
        }
    }
}
