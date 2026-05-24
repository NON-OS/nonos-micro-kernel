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
pub use errno::{E_INVAL, E_IO, E_NODEV};
pub use header::{Request, HDR_LEN, RESP_HDR_LEN};
pub use limits::{
    ADDRESS_DEVICE_REPLY_LEN, ADDRESS_DEVICE_REQUEST_LEN, CONTROLLER_STATUS_PAYLOAD_LEN,
    CONFIG_DESCRIPTOR_REPLY_PREFIX, CONFIG_DESCRIPTOR_REQUEST_LEN, DEVICE_DESCRIPTOR_REPLY_LEN,
    DEVICE_DESCRIPTOR_REQUEST_LEN, MAX_PORTS_REPORTED,
    MAX_REQUEST_PAYLOAD_LEN, PORT_ENTRY_BYTES, PORT_STATUS_HEADER_BYTES, SLOT_DISABLE_PAYLOAD_LEN,
    SLOT_ENABLE_PAYLOAD_LEN, STATUS_LEN,
};
pub use ops::{
    OP_ADDRESS_DEVICE, OP_ALLOC_TRANSFER_RING, OP_BULK_IN, OP_BULK_OUT, OP_CONTROL_TRANSFER,
    OP_CONTROLLER_STATUS, OP_DISABLE_SLOT, OP_ENABLE_SLOT, OP_FREE_TRANSFER_RING,
    OP_GET_CONFIG_DESCRIPTOR, OP_GET_DEVICE_DESCRIPTOR, OP_HEALTHCHECK, OP_INTERRUPT_IN,
    OP_PORT_STATUS,
};