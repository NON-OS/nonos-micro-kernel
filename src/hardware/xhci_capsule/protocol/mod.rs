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

mod codec;
mod header;
mod ops;

pub(super) use codec::{decode_response, encode_request};
pub(super) use header::{CONTROLLER_STATUS_PAYLOAD_LEN, MAX_PORTS_REPORTED};
pub(super) use ops::{
    OP_ADDRESS_DEVICE, OP_CONTROLLER_STATUS, OP_DISABLE_SLOT, OP_ENABLE_SLOT,
    OP_GET_CONFIG_DESCRIPTOR, OP_GET_DEVICE_DESCRIPTOR, OP_HEALTHCHECK, OP_PORT_STATUS,
};
