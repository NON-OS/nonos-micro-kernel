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

// VFS wire protocol plumbing shared by the fs backend: constants, the
// syscall ABI stubs, frame building, the request/reply call, error
// mapping, and the port/path helpers. The std-facing operations live in
// the sibling file/, ops/, and dir modules; this tree exposes only the
// pieces they build requests from.

mod call;
mod consts;
mod err;
mod frame;
mod path;
mod port;
mod syscall;

pub(crate) use call::{call, read_u32};
pub(crate) use consts::{
    BODY_OFF, O_APPEND, O_CREATE, O_TRUNC, OP_CLOSE, OP_LIST, OP_MKDIR, OP_OPEN, OP_READ,
    OP_RENAME, OP_RMDIR, OP_SEEK, OP_STAT, OP_TRUNCATE, OP_UNLINK, OP_WRITE, SEEK_CUR, SEEK_END,
    SEEK_SET,
};
pub(crate) use err::err;
pub(crate) use path::{cwd_bytes, path_body, path_bytes};
pub(crate) use port::{getpid, vfs_port};
