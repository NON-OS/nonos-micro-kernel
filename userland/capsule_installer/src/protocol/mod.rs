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
mod types;

pub use decode::decode_request;
pub use encode::encode_response;
pub use types::{
    Request, EAGAIN, EINVAL, KERNEL_REPLY_ENDPOINT, OP_HEALTHCHECK, OP_INSTALL, OP_LIST_INSTALLED,
    OP_LOAD_BY_NAME, OP_LOAD_FROM_STORE, OP_PKG_COMMIT, OP_PKG_QUERY, OP_PKG_REMOVE,
};
