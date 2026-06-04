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

pub(crate) const N_MK_SURFACE_REGISTER: i64 = tag4(b"MSRG");
pub(crate) const N_MK_SURFACE_SHARE: i64 = tag4(b"MSSH");
pub(crate) const N_MK_SURFACE_ATTACH: i64 = tag4(b"MSAT");
pub(crate) const N_MK_SURFACE_RELEASE: i64 = tag4(b"MSRL");
pub(crate) const N_MK_SURFACE_PRESENT: i64 = tag4(b"MSPR");
pub(crate) const N_MK_DISPLAY_VSYNC_WAIT: i64 = tag4(b"MDVW");
