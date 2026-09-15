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

use super::filetype::Kind;
use super::theme::{AMBER, BLUE, BLUE_LT, CY_DIM, INK, INK2, RED};

/// The filetype tint, drawn from the palette rather than from literals of its
/// own, so `theme` stays the single home for every colour in the app.
pub fn color(kind: Kind) -> u32 {
    match kind {
        Kind::Dir => CY_DIM,
        Kind::Code => BLUE_LT,
        Kind::Image => BLUE,
        Kind::Doc => INK,
        Kind::Archive => AMBER,
        Kind::Exec => RED,
        Kind::Other => INK2,
    }
}
