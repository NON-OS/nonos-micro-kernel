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
mod inbox_name;
mod lookup;
mod pending_reply;
mod recv;
mod recv_from;
mod register;
mod reply;
mod reply_inbox;
mod send;
mod send_caps;
mod send_to_pid;
mod sender_pid;

pub use call::sys_ipc_call;
pub use lookup::sys_service_lookup;
pub(crate) use pending_reply::clear_pid as release_pending_replies_for_pid;
pub use recv::sys_ipc_recv;
pub use recv_from::sys_ipc_recv_from;
pub use register::sys_service_register;
pub use reply::sys_ipc_reply;
pub use send::sys_ipc_send;
pub use send_to_pid::sys_ipc_send_to_pid;
