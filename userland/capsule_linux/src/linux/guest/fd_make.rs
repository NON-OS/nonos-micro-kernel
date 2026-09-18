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


//! The three ways a descriptor comes into being. Kept apart from the type
//! so the fields a caller must fill are a short list in one place, and a
//! field added later cannot be forgotten at one of the call sites.

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

    /// Everything off. A descriptor always leaves here before a maker
    /// sets the fields its kind actually uses.
    pub fn empty(kind: Kind) -> Fd {
        Fd {
            kind,
            offset: 0,
            size: 0,
            path: Vec::new(),
            stream: None,
            pending: Vec::new(),
            names: Vec::new(),
            writable: false,
            handle: 0,
        }
    }
}
