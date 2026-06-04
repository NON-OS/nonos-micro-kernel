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
mod endpoint;
mod errno;
mod header;
mod limits;
mod ops;
pub use decode::decode_request;
pub use encode::{encode_response_header, write_status};
pub use endpoint::KERNEL_REPLY_ENDPOINT;
pub use errno::{E_INVAL, E_IO};
pub use header::{Request, HDR_LEN, RESP_HDR_LEN};
pub use limits::{
    CONTROLLER_STATUS_PAYLOAD_LEN, EVENT_WIRE_LEN, MAX_POLL_EVENTS, MOUSE_EVENT_WIRE_LEN,
    MOUSE_POLL_PAYLOAD_PREFIX_LEN, POLL_PAYLOAD_PREFIX_LEN, STATE_PAYLOAD_LEN, STATUS_LEN,
};
pub use ops::{OP_CONTROLLER_STATUS, OP_GET_STATE, OP_HEALTHCHECK, OP_POLL_EVENTS, OP_POLL_MOUSE};
