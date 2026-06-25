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

use crate::protocol::errno::E_OK;
use crate::server::parse_req::Request;
use crate::server::respond::reply;

pub const OP_HEALTHCHECK: u16 = 1;

pub fn handle(sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let _ = reply(sender_pid, req.magic, req.op, E_OK, req.request_id, &[], tx);
}
