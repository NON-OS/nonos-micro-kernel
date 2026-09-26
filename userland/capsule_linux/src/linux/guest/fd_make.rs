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

//! The three ways a descriptor comes into being.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::VfsStream;

use super::fd::{Fd, Kind};

impl Fd {
    pub fn console(kind: Kind) -> Fd {
        Fd::empty(kind)
    }

    pub fn file(path: Vec<u8>, size: u64, stream: Option<VfsStream>, writable: bool) -> Fd {
        let mut fd = Fd::empty(Kind::File);
        fd.path = path;
        fd.size = size;
        fd.stream = stream;
        fd.writable = writable;
        fd
    }

    pub fn memfd() -> Fd {
        Fd::empty(Kind::Memfd)
    }

    pub fn unix() -> Fd {
        Fd::empty(Kind::Unix)
    }

    pub fn resolver() -> Fd {
        Fd::empty(Kind::Resolver)
    }

    pub fn socket(handle: u32) -> Fd {
        let mut fd = Fd::empty(Kind::Socket);
        fd.handle = handle;
        fd
    }

    pub fn dir(path: Vec<u8>, names: Vec<String>) -> Fd {
        let mut fd = Fd::empty(Kind::Dir);
        fd.path = path;
        fd.names = names;
        fd
    }
}
