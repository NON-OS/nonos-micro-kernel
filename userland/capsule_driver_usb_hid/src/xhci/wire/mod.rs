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

mod constants;
mod parse_response;
mod read_i32;
mod write_request;

pub use constants::{
    E_AGAIN, HDR_LEN, OP_ADDRESS_DEVICE, OP_ALLOC_TRANSFER_RING, OP_CONTROL_TRANSFER,
    OP_ENABLE_SLOT, OP_GET_CONFIG_DESCRIPTOR, OP_INTERRUPT_IN, OP_PORT_STATUS, STATUS_LEN,
};
pub use parse_response::parse_response;
pub use read_i32::read_i32;
pub use write_request::write_request;
