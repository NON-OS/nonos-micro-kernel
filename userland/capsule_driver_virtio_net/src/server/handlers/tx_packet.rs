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

use crate::constants::MAX_ETHERNET_FRAME;
use crate::protocol::{Request, E_INVAL, E_IO, E_MSGSIZE, MAX_TX_PAYLOAD_BYTES};
use crate::server::error::reply_with_status;
use crate::setup::Driver;
use crate::tx::send;

pub fn handle(sender_pid: u32, driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
    if req.payload_len as usize != body.len() {
        return reply_with_status(sender_pid, tx, req, E_MSGSIZE);
    }
    if body.is_empty() || body.len() > MAX_ETHERNET_FRAME {
        return reply_with_status(sender_pid, tx, req, E_INVAL);
    }
    if body.len() as u32 > MAX_TX_PAYLOAD_BYTES {
        return reply_with_status(sender_pid, tx, req, E_MSGSIZE);
    }
    match send(driver.regs, &mut driver.tx, driver.irq_grant, body) {
        Ok(()) => reply_with_status(sender_pid, tx, req, 0),
        Err(_) => reply_with_status(sender_pid, tx, req, E_IO),
    }
}
