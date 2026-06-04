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

use nonos_libc::mk_ipc_reply;

use crate::geometry::Rect;
use crate::protocol::{response_header, write_status, Request, HDR_LEN, WINDOW_OPEN_RESP_LEN};

pub fn window_opened(sender_pid: u32, req: &Request, errno: i32, rect: Rect, tx: &mut [u8]) -> i64 {
    response_header(tx, req, WINDOW_OPEN_RESP_LEN as u32);
    write_status(tx, errno);
    tx[HDR_LEN + 4..HDR_LEN + 8].copy_from_slice(&rect.x.to_le_bytes());
    tx[HDR_LEN + 8..HDR_LEN + 12].copy_from_slice(&rect.y.to_le_bytes());
    tx[HDR_LEN + 12..HDR_LEN + 16].copy_from_slice(&rect.width.to_le_bytes());
    tx[HDR_LEN + 16..HDR_LEN + 20].copy_from_slice(&rect.height.to_le_bytes());
    mk_ipc_reply(sender_pid, tx.as_ptr(), HDR_LEN + WINDOW_OPEN_RESP_LEN)
}
