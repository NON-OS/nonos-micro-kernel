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
mod limits;
mod ops;

pub use errno::{
    E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_PACKET, E_BAD_VERSION, E_L2_FAULT, E_NO_CONFIG,
    E_NO_NEIGHBOUR, E_NO_ROUTE, E_OK, E_PERM, E_RX_EMPTY, E_TABLE_FULL,
};
pub use header::MAGIC;
pub use limits::IPC_PAYLOAD_MAX;
pub use ops::{
    OP_GET_CONFIG, OP_HEALTHCHECK, OP_POLL_PACKET, OP_ROUTE_ADD, OP_ROUTE_CLEAR, OP_SEND_PACKET,
    OP_SET_CONFIG,
};
