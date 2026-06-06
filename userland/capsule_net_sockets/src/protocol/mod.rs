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

mod errno;
mod header;
mod ops;

pub use errno::{
    E_BAD_FAMILY, E_BAD_KIND, E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION, E_NOT_BOUND,
    E_NOT_CONNECTED, E_NO_HANDLE, E_NO_TRANSPORT, E_OK, E_TABLE_FULL,
};
pub use header::MAGIC;
pub use ops::{
    OP_ACCEPT, OP_BIND, OP_CLOSE, OP_CONNECT, OP_GETSOCKOPT, OP_HEALTHCHECK, OP_LISTEN, OP_RECV,
    OP_SEND, OP_SETSOCKOPT, OP_SOCKET,
};
