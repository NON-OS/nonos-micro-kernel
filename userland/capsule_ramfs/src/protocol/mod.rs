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

pub use decode::{decode_request, read_u16_le, read_u32_le, read_u64_le};
pub use encode::encode_response;
pub use errno::{EACCES, EINVAL, EIO, EMFILE, ENOENT};
pub use types::{
    Request, KERNEL_REPLY_ENDPOINT, OPEN_FLAG_CREATE, OPEN_FLAG_TRUNCATE, OP_CLOSE, OP_OPEN,
    OP_READ, OP_TRUNCATE, OP_WRITE,
};
