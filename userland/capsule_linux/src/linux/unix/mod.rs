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


//! Unix domain sockets, both ends inside this capsule.

mod conn;
mod give;
mod msg;
mod msg_parts;
mod msg_rights;
mod path;
mod recvmsg;
mod sendmsg;
mod sock;
mod sock_io;

pub use conn::Conn;
pub use sock::is_unix;
pub use recvmsg::recvmsg;
pub use sendmsg::sendmsg;
pub use sock::{connect, socket};
pub use sock_io::{recv, send};
