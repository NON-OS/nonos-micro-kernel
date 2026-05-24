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

use nonos_libc::{mk_admin_shutdown, mk_time_millis};

use crate::protocol::Request;
use crate::server::respond;
use crate::state::PowerState;

pub fn run(state: &mut PowerState, out: &mut [u8], req: &Request) -> usize {
    let now = mk_time_millis();
    if now > 0 {
        state.last_shutdown_request_unix = now as u64;
    }
    let rc = mk_admin_shutdown();
    let status = if rc == 0 { 0 } else { rc as i32 };
    respond::status(out, req, status)
}
