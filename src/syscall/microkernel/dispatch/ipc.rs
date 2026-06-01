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

use super::args::Args;
use crate::syscall::microkernel::ipc::{
    sys_ipc_call, sys_ipc_recv, sys_ipc_recv_from, sys_ipc_reply, sys_ipc_send,
    sys_ipc_send_to_pid,
    sys_service_lookup, sys_service_register,
};
use crate::syscall::microkernel::numbers::*;

pub(super) fn handle(nr: u64, a: Args) -> Option<i64> {
    Some(match nr {
        SYS_IPC_SEND => sys_ipc_send(a.a0, a.a1, a.a2 as usize),
        SYS_IPC_RECV => sys_ipc_recv(a.a0, a.a1, a.a2 as usize, a.a3),
        SYS_IPC_CALL => sys_ipc_call(a.a0, a.a1, a.a2 as usize, a.a3, a.a4 as usize, a.a5),
        SYS_IPC_RECV_FROM => sys_ipc_recv_from(a.a0, a.a1, a.a2 as usize, a.a3, a.a4),
        SYS_IPC_REPLY => sys_ipc_reply(a.a0, a.a1, a.a2 as usize),
        SYS_IPC_SEND_TO_PID => sys_ipc_send_to_pid(a.a0, a.a1, a.a2 as usize),
        SYS_SERVICE_LOOKUP => sys_service_lookup(a.a0, a.a1 as usize, a.a2, a.a3),
        SYS_SERVICE_REGISTER => sys_service_register(a.a0, a.a1 as usize, a.a2 as u32),
        _ => return None,
    })
}
