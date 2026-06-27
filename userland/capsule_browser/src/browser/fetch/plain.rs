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

use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::http;
use crate::browser::net;

pub fn send_req(state_port: u32, f: &mut Fetch) {
    let req = http::request::build(&f.url);
    if net::socket_send(state_port, f.handle, req.as_bytes()).is_err() {
        f.error = Some("send failed");
        f.phase = Phase::Error;
        return;
    }
    f.buf.clear();
    f.phase = Phase::ReadBody;
}

pub fn read_body(state_port: u32, f: &mut Fetch, tls: bool) -> bool {
    let mut chunk = [0u8; 4096];
    match net::socket_recv(state_port, f.handle, &mut chunk) {
        Ok(n) if n > 0 => {
            f.idle = 0;
            f.buf.extend_from_slice(&chunk[..n]);
            let done = f.buf.len() >= super::MAX || (!tls && http::response::is_complete(&f.buf));
            if done {
                f.phase = if tls { Phase::Decrypt } else { Phase::Done };
            }
        }
        _ => {
            f.idle = f.idle.wrapping_add(1);
            let budget = if f.buf.is_empty() { super::FIRST_WAIT } else { super::IDLE_AFTER };
            if f.idle >= budget {
                f.phase = if tls && !f.buf.is_empty() { Phase::Decrypt } else { Phase::Done };
            }
        }
    }
    true
}
