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

/// Nym request and response protocol version this speaks.
pub const PROTOCOL_VERSION: u8 = 3;

/// Ask the exit to open a TCP connection to the named host.
pub const REQ_CONNECT: u8 = 0;
/// Forward stream bytes, or with the closed flag, close our half.
pub const REQ_SEND: u8 = 1;

/// Stream bytes coming back from the exit.
pub const RESP_NETWORK_DATA: u8 = 1;
/// The exit could not, or can no longer, serve the connection.
pub const RESP_CONNECTION_ERROR: u8 = 2;
