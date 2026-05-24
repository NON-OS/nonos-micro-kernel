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

use crate::protocol::{
    Request, E_INVAL, E_RANGE, MAX_IDLE_TIMEOUT_MS, MIN_IDLE_TIMEOUT_MS,
};
use crate::server::respond;
use crate::state::Clipboard;

pub fn run(clipboard: &mut Clipboard, req: &Request, payload: &[u8], out: &mut [u8]) -> usize {
    if payload.len() < 8 {
        return respond::status(out, req, E_INVAL);
    }
    let raw = u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3],
        payload[4], payload[5], payload[6], payload[7],
    ]);
    if raw != 0 && (raw < MIN_IDLE_TIMEOUT_MS || raw > MAX_IDLE_TIMEOUT_MS) {
        return respond::status(out, req, E_RANGE);
    }
    clipboard.set_idle_timeout_ms(raw);
    respond::status(out, req, 0)
}
