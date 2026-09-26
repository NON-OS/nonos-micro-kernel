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

//! One program header: what to map, from where, and how much of it is in the
//! file rather than in the zeroes after it.

use core::ops::Range;

pub struct Phdr {
    pub kind: u32,
    pub flags: u32,
    pub offset: u64,
    pub vaddr: u64,
    pub filesz: u64,
    pub memsz: u64,
}

impl Phdr {
    /// The bytes of the image this header names.
    pub(super) fn file_range(&self) -> Option<Range<usize>> {
        let from = usize::try_from(self.offset).ok()?;
        let len = usize::try_from(self.filesz).ok()?;
        Some(from..from.checked_add(len)?)
    }

    /// Where this segment lands once the image is biased.
    pub(super) fn at(&self, bias: u64) -> Option<u64> {
        self.vaddr.checked_add(bias)
    }
}
