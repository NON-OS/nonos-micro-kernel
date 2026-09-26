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


//! A pipe end, and a second descriptor onto the same thing.

use super::fd::Fd;
use super::fd_kind::Kind;

impl Fd {
    pub fn pipe(buffer: u32, writable: bool) -> Fd {
        let mut fd = Fd::empty(Kind::Pipe);
        fd.handle = buffer;
        fd.writable = writable;
        fd
    }

    /// A second descriptor onto the same thing, which is what dup is.
    pub fn clone_of(from: &Fd) -> Fd {
        let mut fd = Fd::empty(from.kind);
        fd.handle = from.handle;
        fd.writable = from.writable;
        fd.size = from.size;
        fd.offset = from.offset;
        fd.path = from.path.clone();
        fd
    }
}
