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

//! The per-connection SOCKS5 handshake state machine. It accumulates client
//! bytes, which may arrive split across reads, runs method selection then the
//! CONNECT exchange through the [`crate::wire`] codec, and tells the serving
//! loop what to do next: read more, send a reply, open a tunnel, or close.
//!
//! It owns no transport, so a real socket and a proof drive it the same way.
//! Allocation-free: a client cannot make it grow past one greeting or one
//! request.

mod dest;
mod event;
mod machine;
mod opened;
mod phases;

pub use dest::Dest;
pub use event::Event;
pub use machine::Conn;
