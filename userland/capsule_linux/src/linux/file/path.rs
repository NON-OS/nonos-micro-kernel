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

//! A path out of a guest, which is a string under the store's ceiling.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::cstr::read_cstr;

/// The vfs length prefix is one byte.
pub const MAX_PATH: usize = 255;

pub fn read_path(guest: &Guest, addr: u64) -> Option<Vec<u8>> {
    read_cstr(guest, addr, MAX_PATH)
}
