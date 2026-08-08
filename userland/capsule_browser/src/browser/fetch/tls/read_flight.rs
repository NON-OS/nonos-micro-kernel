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

use crate::browser::fetch::tls::flight_settled;
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::fetch::{append_capped, budget, constants};
use crate::browser::net;
use crate::browser::tls13;

pub(in crate::browser::fetch) fn read_flight(port: u32, f: &mut Fetch) {
    let mut chunk = [0u8; 4096];
    let Some(tls) = f.tls.as_mut() else {
        f.phase = Phase::Error;
        return;
    };
    let mut got = false;
    for _ in 0..constants::DRAIN_BURST {
        match net::socket_recv(port, f.handle, &mut chunk) {
            Ok(n) if n > 0 => {
                got = true;
                if append_capped::append_capped(
                    &mut tls.flight,
                    &chunk[..n],
                    constants::MAX_TLS_FLIGHT,
                )
                .is_err()
                {
                    f.error = Some("tls flight too large");
                    f.phase = Phase::Error;
                    return;
                }
                if tls13::server_finished_flight_ready(&tls.flight) {
                    super::trace::flight(b"complete", tls.flight.len(), f.idle);
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
        if flight_settled(&tls.flight) && f.idle >= budget::flight_settle() {
            // Believed on a quiet gap rather than because the flight said it
            // was done. Over the mixnet the rest can still be on its way, so
            // this is the case worth being able to tell apart.
            super::trace::flight(b"settled", tls.flight.len(), f.idle);
            f.phase = Phase::TlsVerify;
        } else if f.idle >= budget::hs_wait() {
            super::trace::flight(b"abandoned", tls.flight.len(), f.idle);
            f.error = Some("tls handshake failed");
            f.phase = Phase::Error;
        }
    }
}
