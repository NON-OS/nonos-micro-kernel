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

//! What the client has built: its pools, its buffers, its surfaces.

use alloc::vec::Vec;

pub struct Pool {
    pub id: u32,
    /// Where the client's pixels live in its own address space.
    pub at: u64,
    /// How many bytes are really there.
    pub size: u64,
}

pub struct Buffer {
    pub id: u32,
    pub pool: u32,
    pub offset: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

pub struct Surface {
    pub id: u32,
    /// The buffer attached but not yet committed.
    pub pending: Option<u32>,
    /// The xdg_surface wrapping it, once the client asks for one.
    pub xdg: Option<u32>,
    /// Frame callbacks owed, answered after the next present.
    pub frames: Vec<u32>,
    /// Set once a configure has been sent and acked.
    pub configured: bool,
}
