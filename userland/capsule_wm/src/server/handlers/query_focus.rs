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

use crate::protocol::{
    response_header, write_status, Request, HDR_LEN, QUERY_FOCUS_RESP_LEN, STATUS_LEN,
};
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let off = HDR_LEN + STATUS_LEN;
    let focused = ctx.focus.current();
    let values = focused
        .map(|f| [f.owner_pid, f.window_id])
        .unwrap_or([0, 0]);
    for (idx, value) in values.iter().enumerate() {
        let start = off + idx * 4;
        tx[start..start + 4].copy_from_slice(&value.to_le_bytes());
    }
    response_header(tx, req, (STATUS_LEN + QUERY_FOCUS_RESP_LEN) as u32);
    write_status(tx, 0);
    let _ = mk_ipc_reply(sender_pid, tx.as_ptr(), HDR_LEN + STATUS_LEN + QUERY_FOCUS_RESP_LEN);
}
