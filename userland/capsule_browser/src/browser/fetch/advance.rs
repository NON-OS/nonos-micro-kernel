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

use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::fetch::{constants, plain, rtc_packed, socks, tls};
use crate::browser::http;

// Drive one fetch through the next step of its state machine. Returns true
// while the fetch is still progressing (handshake or body read) and false once
// it reaches a terminal phase (Decrypt/Done/Error) and is ready to be
// completed by the caller. The same machine backs the page fetch and every
// concurrent image fetch, so neither needs its own copy of the phase logic.
pub(crate) fn advance(port: u32, f: &mut Fetch) -> bool {
    let now = rtc_packed::rtc_packed();
    if mk_time_millis().wrapping_sub(f.started_ms) > constants::MAX_FETCH_MS {
        if f.tls.is_some() {
            f.phase = if tls::decrypt(f).is_some_and(|p| http::response::has_headers(&p)) {
                Phase::Decrypt
            } else {
                Phase::Error
            };
        } else {
            f.phase = if f.buf.is_empty() { Phase::Error } else { Phase::Done };
        }
        if f.error.is_none() && matches!(f.phase, Phase::Error) {
            f.error = Some("timed out");
        }
    }
    match f.phase {
        Phase::SocksHello => {
            socks::hello(port, f);
        }
        Phase::SocksMethod => {
            socks::method(port, f);
        }
        Phase::SocksConnect => {
            socks::connect(port, f);
        }
        Phase::TlsHello => {
            tls::hello(port, f, now);
        }
        Phase::TlsFlight => {
            tls::read_flight(port, f);
        }
        Phase::TlsVerify => {
            tls::verify_and_send(port, f);
        }
        Phase::SendReq => {
            plain::send_req(port, f);
        }
        Phase::ReadBody => {
            plain::read_body(port, f, f.tls.is_some());
        }
        Phase::Decrypt | Phase::Done | Phase::Error => return false,
    }
    true
}
