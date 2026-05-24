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

use super::{health, reboot, shutdown};
use crate::protocol::{parse, E_BAD_OP, OP_HEALTHCHECK, OP_REBOOT, OP_SHUTDOWN};
use crate::server::respond;
use crate::state::PowerState;

pub fn route(state: &mut PowerState, input: &[u8], out: &mut [u8]) -> usize {
    let (req, _payload) = match parse(input) {
        Ok(parsed) => parsed,
        Err((req, status)) => return respond::status(out, &req, status),
    };
    match req.op {
        OP_HEALTHCHECK => health::run(out, &req),
        OP_REBOOT => reboot::run(state, out, &req),
        OP_SHUTDOWN => shutdown::run(state, out, &req),
        _ => respond::status(out, &req, E_BAD_OP),
    }
}
