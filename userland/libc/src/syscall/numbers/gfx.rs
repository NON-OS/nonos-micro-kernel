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
use super::tag::tag4;

pub(crate) const N_GFX_DISPLAY_DIMENSIONS: i64 = tag4(b"GDIM");
pub(crate) const N_GFX_SURFACE_CREATE: i64 = tag4(b"GSCR");
pub(crate) const N_GFX_SURFACE_DESTROY: i64 = tag4(b"GSDS");
pub(crate) const N_GFX_SURFACE_MAP: i64 = tag4(b"GSMP");
pub(crate) const N_GFX_SURFACE_PRESENT_FULL: i64 = tag4(b"GPRF");
pub(crate) const N_GFX_SURFACE_PRESENT_RECT: i64 = tag4(b"GPRR");
pub(crate) const N_GFX_DISPLAY_LIST: i64 = tag4(b"GDLS");
pub(crate) const N_GFX_CURSOR_PRESENT: i64 = tag4(b"GCUR");
