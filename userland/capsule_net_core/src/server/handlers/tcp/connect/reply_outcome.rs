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

use crate::protocol::tcp::{E_NOT_CONNECTED, E_NO_SOCKET, E_OK, MAGIC_NTCP, OP_CONNECT};
use crate::server::handlers::tcp::connect::types::ConnectOutcome;
use crate::server::respond::reply;

pub fn reply_outcome(sender_pid: u32, request_id: u32, outcome: ConnectOutcome, tx: &mut [u8]) {
    match outcome {
        ConnectOutcome::Ok(app_handle) => {
            let _ = reply(
                sender_pid,
                MAGIC_NTCP,
                OP_CONNECT,
                E_OK,
                request_id,
                &app_handle.to_le_bytes(),
                tx,
            );
        }
        ConnectOutcome::ConnectFailed => {
            let _ = reply(sender_pid, MAGIC_NTCP, OP_CONNECT, E_NOT_CONNECTED, request_id, &[], tx);
        }
        ConnectOutcome::TableFull => {
            let _ = reply(sender_pid, MAGIC_NTCP, OP_CONNECT, E_NO_SOCKET, request_id, &[], tx);
        }
    }
}
