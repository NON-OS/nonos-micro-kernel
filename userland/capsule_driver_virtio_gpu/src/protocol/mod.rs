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
mod read;
pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{E_BAD_OP, E_BUSY, E_DEVICE, E_INVAL, E_NOMEM};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{
    ATTACH_BACKING_REQ_LEN, CONTROLLER_INFO_LEN, CONTROLQ_STATE_LEN, CREATE_RESOURCE_REQ_LEN,
    DISPLAY_INFO_LEN, FLUSH_REQ_LEN, GET_PRIMARY_SURFACE_RESP_LEN, IPC_PAYLOAD_MAX, MAX_RESOURCES,
    MODE_LIST_ENTRY_LEN, QUERY_CAPS_RESP_LEN, SET_SCANOUT_REQ_LEN, STATUS_LEN,
    TRANSFER_TO_HOST_REQ_LEN,
};
pub use ops::{
    OP_ATTACH_BACKING, OP_CONTROLLER_INFO, OP_CONTROLQ_STATE, OP_CREATE_RESOURCE, OP_DISPLAY_INFO,
    OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_HEALTHCHECK, OP_MODE_LIST, OP_QUERY_CAPS, OP_SET_SCANOUT,
    OP_TRANSFER_TO_HOST,
};
pub use read::{le_u32, le_u64};
