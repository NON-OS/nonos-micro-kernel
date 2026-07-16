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

use crate::bot;
use crate::protocol::Request;
use crate::server::respond;
use crate::state::State;

pub fn handle(state: &mut State, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    match bot::parse(body) {
        Ok(csw) => match state.finish_command(csw) {
            Ok(status) => {
                let _ = respond::status(sender_pid, req, i32::from(status), tx);
            }
            Err(errno) => {
                let _ = respond::status(sender_pid, req, errno, tx);
            }
        },
        Err(errno) => {
            let _ = respond::status(sender_pid, req, errno, tx);
        }
    }
}
