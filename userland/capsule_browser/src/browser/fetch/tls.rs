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

use crate::browser::fetch::types::{Fetch, Phase, TlsCtx};
use crate::browser::http;
use crate::browser::net;
use crate::browser::tls13;

pub fn hello(port: u32, f: &mut Fetch, now: u64) {
    let host = f.url.host.clone();
    let Some(cf) = tls13::client_flight(host.as_bytes()) else {
        f.error = Some("tls init failed"); f.phase = Phase::Error; return;
    };
    if net::socket_send(port, f.handle, &cf.record).is_err() {
        f.error = Some("send failed"); f.phase = Phase::Error; return;
    }
    f.tls = Some(TlsCtx { cf, flight: alloc::vec::Vec::new(), now });
    f.phase = Phase::TlsFlight;
}

pub fn read_flight(port: u32, f: &mut Fetch) {
    let mut chunk = [0u8; 4096];
    let Some(tls) = f.tls.as_mut() else { f.phase = Phase::Error; return; };
    let mut got = false;
    for _ in 0..super::DRAIN_BURST {
        match net::socket_recv(port, f.handle, &mut chunk) {
            Ok(n) if n > 0 => {
                got = true;
                tls.flight.extend_from_slice(&chunk[..n]);
                if tls13::server_finished_flight_ready(&tls.flight) {
                    f.phase = Phase::TlsVerify;
                    return;
                }
            }
            _ => break,
        }
    }
    if got {
        f.idle = 0;
    } else {
        f.idle = f.idle.wrapping_add(1);
        if flight_settled(&tls.flight) && f.idle >= super::FLIGHT_SETTLE {
            f.phase = Phase::TlsVerify;
        } else if f.idle >= super::HS_WAIT {
            f.error = Some("tls handshake failed");
            f.phase = Phase::Error;
        }
    }
}

fn flight_settled(bytes: &[u8]) -> bool {
    let mut pos = 0usize;
    while pos + 5 <= bytes.len() {
        let len = u16::from_be_bytes([bytes[pos + 3], bytes[pos + 4]]) as usize;
        let end = pos + 5 + len;
        if end > bytes.len() {
            return false;
        }
        if bytes[pos] == 23 {
            return true;
        }
        pos = end;
    }
    false
}

pub fn verify_and_send(port: u32, f: &mut Fetch) {
    let req = http::request::build(&f.url);
    let host = f.url.host.clone();
    let Some(tls) = f.tls.as_ref() else { f.phase = Phase::Error; return; };
    let Some(out) = tls13::application_write(&tls.cf, &tls.flight, req.as_bytes(), host.as_bytes(), tls.now) else {
        f.error = Some("cert verify failed"); f.phase = Phase::Error; return;
    };
    if net::socket_send(port, f.handle, &out).is_err() {
        f.error = Some("send failed"); f.phase = Phase::Error; return;
    }
    f.buf.clear();
    f.phase = Phase::ReadBody;
}

pub fn decrypt(f: &Fetch) -> Option<alloc::vec::Vec<u8>> {
    let tls = f.tls.as_ref()?;
    tls13::application_plaintext(&tls.cf, &tls.flight, &f.buf, f.url.host.as_bytes(), tls.now)
}
