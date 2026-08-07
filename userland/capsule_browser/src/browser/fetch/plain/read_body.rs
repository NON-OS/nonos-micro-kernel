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
use crate::browser::fetch::{append_capped, budget, constants, tls};
use crate::browser::http;
use crate::browser::net;

pub(in crate::browser::fetch) fn read_body(state_port: u32, f: &mut Fetch, tls_mode: bool) -> bool {
    let mut chunk = [0u8; 4096];
    let mut got = false;
    for _ in 0..constants::DRAIN_BURST {
        match net::socket_recv(state_port, f.handle, &mut chunk) {
            Ok(n) if n > 0 => {
                got = true;
                if append_capped::append_capped(&mut f.buf, &chunk[..n], constants::MAX_BODY)
                    .is_err()
                {
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
        // A kept connection the server already dropped never answers, and the
        // idle accounting below only starts once headers exist. Fail a silent
        // reused connection after the empty-buffer budget so the caller
        // retries on a fresh one instead of waiting out the fetch timeout.
        if f.keep_uses > 0 && f.buf.is_empty() {
            if got {
                f.idle = 0;
            } else {
                f.idle = f.idle.wrapping_add(1);
                if f.idle >= budget::first_wait() {
                    f.error = Some("kept connection dead");
                    f.phase = Phase::Error;
                    return true;
                }
            }
        }
        // Decrypt once per tick and judge completion on the plaintext directly.
        // Checking every tick (not only once the raw socket goes quiet) lets a
        // content-length or chunked response finish the instant its last byte
        // decrypts, instead of hanging until the fetch timeout.
        if let Some(plain) = tls::decrypt(f) {
            if http::response::has_headers(&plain) {
                if http::response::is_complete(&plain) {
                    f.phase = Phase::Decrypt;
                    return true;
                }
                // Keep-alive responses we cannot frame end when the decrypted
                // body stops growing; track the plaintext length, not the raw
                // socket, so post-handshake records do not reset the idle count.
                if plain.len() as usize > f.last_check as usize {
                    f.last_check = plain.len();
                    f.idle = 0;
                } else {
                    f.idle = f.idle.wrapping_add(1);
                    if f.idle >= budget::idle_after() {
                        f.phase = Phase::Decrypt;
                        return true;
                    }
                }
            }
        }
        if f.buf.len() >= constants::MAX_BODY {
            f.error = Some("response too large");
            f.phase = Phase::Error;
        }
        return true;
    }
    if got {
        f.idle = 0;
    } else {
        f.idle = f.idle.wrapping_add(1);
        let allowed = if f.buf.is_empty() { budget::first_wait() } else { budget::idle_after() };
        if f.idle >= allowed {
            f.phase = Phase::Done;
        }
    }
    true
}
