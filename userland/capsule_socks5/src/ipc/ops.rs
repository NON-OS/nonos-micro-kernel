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

/// Open a session on the mixnet transport.
pub const OP_OPEN_SESSION: u16 = 3;
/// Send a payload through an open session.
pub const OP_SEND: u16 = 4;
/// Collect anything the mixnet delivered.
pub const OP_RECV: u16 = 5;
/// Close a session, freeing the single destination slot it held.
pub const OP_CLOSE: u16 = 7;
/// Bind a session to the Nym address its traffic is sealed for.
pub const OP_SET_DESTINATION: u16 = 17;

/// Ask net.nym for an exit it found in the directory, so this capsule does
/// not have to carry one compiled in.
pub const OP_GET_EXIT: u16 = 19;
