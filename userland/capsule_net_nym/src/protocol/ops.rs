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

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_SET_GATEWAY: u16 = 2;
pub const OP_OPEN_SESSION: u16 = 3;
pub const OP_SEND: u16 = 4;
pub const OP_RECV: u16 = 5;
pub const OP_COVER_TICK: u16 = 6;
pub const OP_CLOSE: u16 = 7;
pub const OP_SET_TOPOLOGY: u16 = 8;
pub const OP_SET_CREDENTIAL: u16 = 9;
pub const OP_CREATE_SURB: u16 = 10;
pub const OP_SEND_REPLY: u16 = 11;
pub const OP_SET_TIMING: u16 = 12;
pub const OP_SET_AUTHORITY: u16 = 13;
pub const OP_SYNC_DIRECTORY: u16 = 14;
pub const OP_TOPOLOGY_STATUS: u16 = 15;
pub const OP_TIMING_STATUS: u16 = 16;

/// Bind a session to a Nym destination so its traffic is sealed as Sphinx.
pub const OP_SET_DESTINATION: u16 = 17;

/// Install the Ed25519 identity the gateway handshake signs with.
pub const OP_SET_IDENTITY: u16 = 18;

/// Ask for an exit the directory published, so a client does not have to
/// carry one compiled in.
pub const OP_GET_EXIT: u16 = 19;
