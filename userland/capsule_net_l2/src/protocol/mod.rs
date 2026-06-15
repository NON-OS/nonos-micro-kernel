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
pub use encode::write_header;
pub use errno::{E_BAD_LEN, E_BAD_OP, E_NO_LINK, E_NO_NEIGHBOUR, E_OK, E_RX_EMPTY, E_TX_BUSY};
pub use header::{Request, HDR_LEN};
pub use limits::IPC_PAYLOAD_MAX;
pub use ops::{
    OP_ARP_RESOLVE, OP_GET_LINK, OP_GET_MAC, OP_HEALTHCHECK, OP_POLL_FRAME, OP_SEND_FRAME,
    OP_SET_IP,
};
