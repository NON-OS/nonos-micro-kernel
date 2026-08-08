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
//! Pack files: the format a fetch or clone sends objects in.
//!
//! A pack is a header, a run of zlib-compressed objects, and a SHA-1 trailer.
//! Most objects arrive as deltas against another object in the same pack, so
//! reading one means resolving those chains before anything can be stored.

mod delta;
mod entry;
mod error;
mod header;
mod reader;
mod varint;
mod write;

pub use error::PackError;
pub use reader::{read_pack, PackObject};
pub use write::write_pack;
