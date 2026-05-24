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
mod header;
mod limits;
mod ops;

pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION, E_INVAL, E_RANGE};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{
    DEFAULT_IDLE_TIMEOUT_MS, IPC_PAYLOAD_MAX, MAX_DEPTH, MAX_ENTRY_BYTES, MAX_IDLE_TIMEOUT_MS,
    MAX_TOTAL_BYTES, MIN_IDLE_TIMEOUT_MS, STATUS_LEN,
};
pub use ops::{
    OP_CLEAR, OP_COPY, OP_HEALTHCHECK, OP_HISTORY_GET, OP_HISTORY_LIST, OP_PASTE,
    OP_SET_IDLE_TIMEOUT,
};
