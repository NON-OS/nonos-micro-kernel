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

use crate::protocol::{E_BAD_LEN, E_NO_SOCKET, E_OK, OP_CLOSE};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::server::tcp_tx;
use crate::state::TABLE;
use crate::tcp::{State, FLAG_ACK, FLAG_FIN};

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let handle = match u32_at(body, 0) {
        Ok(h) => h,
        Err(_) => return status(sender_pid, req, E_BAD_LEN, tx),
    };
    let mut table = TABLE.lock();
    let info = table.owned_mut(sender_pid, handle).map(|e| {
        let prev = e.tcb.state;
        let fin_tcb = e.tcb;
        if prev == State::Established || prev == State::CloseWait {
            e.tcb.send.nxt = e.tcb.send.nxt.wrapping_add(1);
            e.tcb.state = if prev == State::Established { State::FinWait1 } else { State::LastAck };
        }
        (prev, fin_tcb)
    });
    match info {
        None => {
            drop(table);
            status(sender_pid, req, E_NO_SOCKET, tx);
        }
        Some((prev, tcb)) => {
            let closing = prev == State::Established || prev == State::CloseWait;
            if !closing {
                let _ = table.remove(sender_pid, handle);
            }
            drop(table);
            if closing {
                let _ = tcp_tx::send(tcb, FLAG_ACK | FLAG_FIN, &[]);
            }
            let _ = respond(sender_pid, OP_CLOSE, E_OK, req.request_id, 0, tx);
        }
    }
}

fn status(sender_pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    let _ = respond(sender_pid, OP_CLOSE, errno, req.request_id, 0, tx);
}
