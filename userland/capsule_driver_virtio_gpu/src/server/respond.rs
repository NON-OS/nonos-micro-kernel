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
use nonos_libc::mk_ipc_send_to_pid;
use crate::protocol::{response_header, write_status, Request, HDR_LEN, STATUS_LEN};
pub fn status(sender_pid: u32, req: &Request, errno: i32, tx: &mut [u8]) -> i64 {
    response_header(tx, req, STATUS_LEN as u32);
    write_status(tx, errno);
    mk_ipc_send_to_pid(sender_pid, tx.as_ptr(), HDR_LEN + STATUS_LEN)
}
pub fn payload(sender_pid: u32, req: &Request, body_len: usize, tx: &mut [u8]) -> i64 {
    response_header(tx, req, (STATUS_LEN + body_len) as u32);
    write_status(tx, 0);
    mk_ipc_send_to_pid(sender_pid, tx.as_ptr(), HDR_LEN + STATUS_LEN + body_len)
}