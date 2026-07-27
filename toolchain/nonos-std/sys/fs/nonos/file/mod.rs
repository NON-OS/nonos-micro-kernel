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

// The open-file handle. The struct lives in `handle`; its operations are
// split one concern to a file (open, read, write, seek, lock, truncate,
// metadata, and the descriptor plumbing `os::fd` reaches through) and hang
// off it as impl blocks. Close is RAII through the descriptor table, which
// issues the vfs OP_CLOSE when the last descriptor over the handle drops.

mod fd;
mod handle;
mod lock;
mod meta;
mod open;
mod read;
mod seek;
mod truncate;
mod write;

pub use handle::File;
