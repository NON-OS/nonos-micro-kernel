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

use crate::protocol::OP_CONNECT_HOST;
use crate::server::parse_req::Request;
use crate::server::respond::respond;

pub fn status(pid: u32, req: &Request, errno: u16, tx: &mut [u8]) {
    respond(pid, OP_CONNECT_HOST, errno, req.request_id, 0, tx);
}
