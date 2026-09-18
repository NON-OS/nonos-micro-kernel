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


//! What the loader learned about an image, which is everything the
//! auxiliary vector has to carry and the interpreter has to be told.

use alloc::vec::Vec;

pub struct Loaded {
    /// Where execution starts, with the load bias already applied.
    pub entry: u64,
    /// Where the program headers landed, or zero when they are in no
    /// load segment and the guest therefore cannot read them.
    pub phdr: u64,
    pub phentsize: u64,
    pub phnum: u64,
    /// The interpreter this image asked for, if it asked for one.
    pub interp: Option<Vec<u8>>,
    /// The bias every address in the image was shifted by.
    pub bias: u64,
}

pub enum LoadError {
    NotElf,
    Map,
    Copy,
    /// An interpreter was named and could not be read or parsed. The
    /// program cannot start without it, and starting it anyway would
    /// fault on the first unresolved call.
    Interp,
}
