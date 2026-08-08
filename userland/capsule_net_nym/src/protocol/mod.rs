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
    E_AUTHORITY_MISSING, E_AUTHORITY_UNTRUSTED, E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION,
    E_BUSY, E_CREDENTIAL_EXPIRED, E_CRYPTO, E_DIRECTORY_PROTO, E_DIRECTORY_SOURCE, E_GATEWAY_PROTO,
    E_NO_CREDENTIAL, E_NO_GATEWAY, E_NO_ROUTE, E_NO_SESSION, E_NO_TCP, E_NO_TOPOLOGY, E_OK, E_PERM,
    E_RX_EMPTY, E_TABLE_FULL, E_TOPOLOGY_AUTH, E_TOPOLOGY_EXPIRED, E_TOPOLOGY_STALE,
};
pub use header::MAGIC;
pub use limits::{
    COVER_BYTES, IPC_PAYLOAD_MAX, MIX_PAYLOAD_MAX, NYM_HEADER_BYTES, NYM_PAYLOAD_BYTES,
    WIRE_PACKET_MAX,
};
pub use ops::{
    OP_CLOSE, OP_COVER_TICK, OP_CREATE_SURB, OP_GET_EXIT, OP_HEALTHCHECK, OP_OPEN_SESSION, OP_RECV,
    OP_SEND, OP_SEND_REPLY, OP_SET_AUTHORITY, OP_SET_CREDENTIAL, OP_SET_DESTINATION,
    OP_SET_GATEWAY, OP_SET_IDENTITY, OP_SET_TIMING, OP_SET_TOPOLOGY, OP_SYNC_DIRECTORY,
    OP_TIMING_STATUS, OP_TOPOLOGY_STATUS,
};
