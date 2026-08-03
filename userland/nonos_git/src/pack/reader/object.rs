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
//! One object out of a pack.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::ObjectKind;
use crate::oid::ObjectId;

pub struct PackObject {
    pub id: ObjectId,
    pub kind: ObjectKind,
    /// Full content, with any delta already applied.
    pub data: Vec<u8>,
    /// Byte offset in the pack, which an ofs-delta counts back from.
    pub offset: usize,
}
