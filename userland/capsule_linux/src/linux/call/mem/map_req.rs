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


//! What a guest asked `mmap` for, in one value.

pub struct MapReq {
    pub addr: u64,
    pub len: u64,
    pub prot: u64,
    pub flags: u64,
    pub fd: u64,
    pub off: u64,
}

impl MapReq {
    pub fn from_args(a: [u64; 6]) -> MapReq {
        MapReq { addr: a[0], len: a[1], prot: a[2], flags: a[3], fd: a[4], off: a[5] }
    }

    /// The address the guest named, or nothing when it left the choice
    /// to this capsule, which is the case the mapping cursor advances on.
    pub fn fixed(&self) -> Option<u64> {
        match self.addr {
            0 => None,
            addr => Some(crate::linux::guest::page_down(addr)),
        }
    }
}
