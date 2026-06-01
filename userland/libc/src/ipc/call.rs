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

use crate::syscall::{call_raw, N_MK_IPC_CALL};

#[no_mangle]
pub extern "C" fn mk_ipc_call(
    endpoint: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    resp_len: usize,
) -> i64 {
    call_raw(N_MK_IPC_CALL, [endpoint, req as u64, req_len as u64, resp as u64, resp_len as u64, 0])
}

#[no_mangle]
pub extern "C" fn mk_ipc_call_timeout(
    endpoint: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    resp_len: usize,
    timeout_ms: u64,
) -> i64 {
    call_raw(
        N_MK_IPC_CALL,
        [endpoint, req as u64, req_len as u64, resp as u64, resp_len as u64, timeout_ms],
    )
}
