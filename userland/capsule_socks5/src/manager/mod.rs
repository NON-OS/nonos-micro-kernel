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

//! Track the tunneled connections. One mixnet session carries every client's
//! traffic, so each SOCKS5 client that reaches the relay phase gets a
//! connection id its tunnel frames carry. A response off the mixnet names an
//! id; the manager maps it back to the client socket to write, and the reverse
//! when a client socket closes. Fixed capacity, no allocation: the server
//! refuses a new client rather than growing without bound.

mod lookup;
mod seq;
mod table;

pub use table::{Manager, MAX_CONNS};
