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


//! Sockets, over the net.sockets service. A guest's descriptor holds a
//! handle that service issued to this capsule, so a guest reaches only
//! the sockets this capsule opened for it.

mod addr;
mod call;
mod connect;
mod ops;
mod poll;
mod poll_set;
mod socket;
mod stream;

pub use connect::connect;
pub use poll_set::poll;
pub use socket::socket;
pub use stream::{close, recv, send};
