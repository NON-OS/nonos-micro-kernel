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
pub use errno::{E_BAD_OP, E_INVAL, E_NO_MSC, E_OVERFLOW, E_PHASE};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{
    BLOCK_BYTES, CBW_LEN, CSW_LEN, IPC_PAYLOAD_MAX, MAX_BINDINGS, MAX_TRANSFER_BLOCKS, STATUS_LEN,
};
pub use ops::{
    OP_ACCEPT_CSW, OP_BUILD_INQUIRY, OP_BUILD_READ10, OP_BUILD_READ_CAPACITY10,
    OP_BUILD_REQUEST_SENSE, OP_BUILD_TEST_UNIT_READY, OP_BUILD_WRITE10, OP_DECODE_CAPACITY,
    OP_DECODE_INQUIRY, OP_DECODE_SENSE, OP_GET_STATE, OP_HEALTHCHECK, OP_PROBE_CONFIG,
};
