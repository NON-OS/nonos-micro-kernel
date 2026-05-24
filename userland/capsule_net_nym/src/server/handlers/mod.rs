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

mod close;
mod cover;
mod dispatch;
mod gateway;
mod health;
mod io;
mod open;
mod recv;
mod recv_drain;
mod recv_plain;
mod send;
mod send_reply;
mod set_authority;
mod set_credential;
mod set_timing;
mod set_topology;
mod surb;
mod sync_directory;
mod timing_status;
mod topology_errno;
mod topology_status;

pub use dispatch::dispatch;
