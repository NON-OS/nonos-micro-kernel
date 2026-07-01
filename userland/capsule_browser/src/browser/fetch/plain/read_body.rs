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

use crate::browser::fetch::{append_capped, constants, tls};
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::http;
use crate::browser::net;

pub(in crate::browser::fetch) fn read_body(state_port: u32, f: &mut Fetch, tls_mode: bool) -> bool {
    let mut chunk = [0u8; 4096];
    let mut got = false;
    for _ in 0..constants::DRAIN_BURST {
        match net::socket_recv(state_port, f.handle, &mut chunk) {
            Ok(n) if n > 0 => {
                got = true;
                if append_capped::append_capped(&mut f.buf, &chunk[..n], constants::MAX_BODY).is_err() {
                    f.error = Some("response too large");
                    f.phase = Phase::Error;
                    return true;
                }
                if !tls_mode && http::response::is_complete(&f.buf) {
                    f.phase = Phase::Done;
                    return true;
                }
            }
            _ => break,
        }
    }
    if tls_mode {
        let grew = f.buf.len() > f.last_check;
        if grew && (!got || f.buf.len() >= f.last_check + constants::CHECK_STRIDE) {
            f.last_check = f.buf.len();
            if tls::decrypt(f).map_or(false, |p| http::response::is_complete(&p)) {
                f.phase = Phase::Decrypt;
            }
        }
        if f.buf.len() == constants::MAX_BODY && !matches!(f.phase, Phase::Decrypt) {
            f.error = Some("response too large");
            f.phase = Phase::Error;
        }
        return true;
    }
    if got {
        f.idle = 0;
    } else {
        f.idle = f.idle.wrapping_add(1);
        let budget = if f.buf.is_empty() { constants::FIRST_WAIT } else { constants::IDLE_AFTER };
        if f.idle >= budget { f.phase = Phase::Done; }
    }
    true
}
