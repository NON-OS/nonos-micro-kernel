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

/// A point in the unit box: both axes run 0..=1000, so a glyph is resolution
/// independent and the stroker does the only division.
pub type Pt = (i16, i16);

/// A ring in the unit box: centre plus radius, all in the same 0..=1000 space.
pub type Ring = (i16, i16, i16);

/// One glyph: open polylines that get stroked end to end, plus rings the
/// stroker hands straight to the toolkit's own circle primitive. A closed shape
/// simply repeats its first point last.
pub struct Glyph {
    pub polys: &'static [&'static [Pt]],
    pub rings: &'static [Ring],
}

#[derive(Clone, Copy, PartialEq)]
pub enum Icon {
    Home,
    Clock,
    Star,
    People,
    Tag,
    Download,
    Doc,
    Folder,
    Cube,
    Archive,
    Trash,
    Chevron,
    Magnifier,
    Grid,
    List,
    Sliders,
    Plus,
    Back,
    Forward,
    Check,
    Ellipsis,
}
