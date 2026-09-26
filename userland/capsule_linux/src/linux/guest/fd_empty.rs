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

//! A descriptor with every field off, which every maker starts from.

use alloc::vec::Vec;

use super::fd::Fd;
use super::fd_kind::Kind;

impl Fd {
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
            watch: Vec::new(),
            expiry: 0,
            replies: Vec::new(),
            cloexec: false,
        }
    }
}
