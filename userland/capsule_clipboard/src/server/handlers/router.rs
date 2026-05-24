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

use super::{clear, copy, health, history_get, history_list, paste, set_idle_timeout};
use crate::protocol::{
    parse, OP_CLEAR, OP_COPY, OP_HEALTHCHECK, OP_HISTORY_GET, OP_HISTORY_LIST, OP_PASTE,
    OP_SET_IDLE_TIMEOUT,
};
use crate::server::respond;
use crate::state::Clipboard;

pub fn route(clipboard: &mut Clipboard, input: &[u8], out: &mut [u8], now_ms: u64) -> usize {
    let (req, payload) = match parse(input) {
        Ok(parsed) => parsed,
        Err((req, status)) => return respond::status(out, &req, status),
    };
    match req.op {
        OP_HEALTHCHECK => health::run(out, &req),
        OP_COPY => copy::run(clipboard, &req, payload, out, now_ms),
        OP_PASTE => paste::run(clipboard, &req, payload, out, now_ms),
        OP_HISTORY_LIST => history_list::run(clipboard, &req, out),
        OP_HISTORY_GET => history_get::run(clipboard, &req, payload, out, now_ms),
        OP_CLEAR => clear::run(clipboard, &req, out, now_ms),
        OP_SET_IDLE_TIMEOUT => set_idle_timeout::run(clipboard, &req, payload, out),
        _ => respond::status(out, &req, crate::protocol::E_BAD_OP),
    }
}
