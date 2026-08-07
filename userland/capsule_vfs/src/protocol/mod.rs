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

mod decode;
mod encode;
mod errno;
mod types;

pub use decode::decode_request;
pub use encode::encode_response;
pub use errno::{
    EACCES, EBADF, EEXIST, EINVAL, EISDIR, EMSGSIZE, ENOENT, ENOSPC, ENOTEMPTY,
};
pub use types::{
    Request, KERNEL_REPLY_ENDPOINT, MAX_DATA_BYTES, MAX_LIST_BYTES, MAX_PATH_BYTES, OP_CHMOD,
    OP_CLOSE, OP_COPY, OP_HEALTHCHECK, OP_LIST, OP_MKDIR, OP_OPEN, OP_READ, OP_RENAME, OP_RMDIR,
    OP_SEEK, OP_STAT, OP_STORE_INSTALL, OP_STORE_PERSIST, OP_STORE_REMOVE, OP_STORE_STATUS,
    OP_STORE_UNINSTALL, OP_TRUNCATE, OP_UNLINK,
    OP_USAGE, OP_WRITE, O_APPEND,
    O_CREATE, O_TRUNC,
    SEEK_CUR, SEEK_END, SEEK_SET, STORE_INSTALL_FINAL,
};
