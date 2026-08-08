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
//! Talking to the net.sockets capsule.
//!
//! A TCP stream here is a handle the sockets capsule owns, driven by request
//! and reply over IPC. Nothing in this crate touches a device or a packet; it
//! is the client side of one service boundary and no more.

#![no_std]

extern crate alloc;

mod call;
mod constants;
mod error;
mod lookup;
mod op;
mod stream;

pub use error::SocketError;
pub use lookup::lookup;
pub use op::{close, connect_host, open, recv, send};
pub use stream::TcpStream;
