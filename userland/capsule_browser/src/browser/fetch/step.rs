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

use nonos_libc::mk_time_millis;

use crate::browser::fetch::{constants, fail, finish, plain, rtc_packed, socks, tls};
use crate::browser::fetch::types::Phase;
use crate::browser::net;
use crate::browser::state::State;

pub fn step(state: &mut State) -> bool {
    let port = state.sockets_port;
    let now = rtc_packed::rtc_packed();
    {
        let Some(f) = state.fetch.as_mut() else { return false; };
        if mk_time_millis().wrapping_sub(f.started_ms) > constants::MAX_FETCH_MS {
            f.phase = if f.buf.is_empty() { Phase::Error } else { Phase::Done };
            if f.error.is_none() && f.buf.is_empty() { f.error = Some("timed out"); }
        }
        match f.phase {
            Phase::SocksHello => { socks::hello(port, f); return true; }
            Phase::SocksMethod => { socks::method(port, f); return true; }
            Phase::SocksConnect => { socks::connect(port, f); return true; }
            Phase::TlsHello => { tls::hello(port, f, now); return true; }
            Phase::TlsFlight => { tls::read_flight(port, f); return true; }
            Phase::TlsVerify => { tls::verify_and_send(port, f); return true; }
            Phase::SendReq => { plain::send_req(port, f); return true; }
            Phase::ReadBody => { plain::read_body(port, f, f.tls.is_some()); return true; }
            Phase::Decrypt | Phase::Done | Phase::Error => {}
        }
    }
    let Some(job) = state.fetch.take() else { return false; };
    let _ = net::socket_close(port, job.handle);
    match job.phase {
        Phase::Decrypt => match tls::decrypt(&job) {
            Some(p) => finish::finish(state, &p, job.suppress),
            None => fail::fail(state, "decrypt failed"),
        },
        Phase::Done => finish::finish(state, &job.buf, job.suppress),
        _ => fail::fail(state, match job.error {
            Some(err) => err,
            None => "error",
        }),
    }
    true
}
