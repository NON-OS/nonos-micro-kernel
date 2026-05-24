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

mod call;
mod lookup;
mod recv;
mod recv_from;
mod register;
mod send;
mod send_to_pid;

pub use call::mk_ipc_call;
pub use lookup::mk_service_lookup;
pub use recv::mk_ipc_recv;
pub use recv_from::mk_ipc_recv_from;
pub use register::mk_service_register;
pub use send::mk_ipc_send;
pub use send_to_pid::mk_ipc_send_to_pid;
