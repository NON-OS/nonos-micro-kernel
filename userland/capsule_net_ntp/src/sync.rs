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

use nonos_libc::{mk_time_adjust, mk_time_millis, mk_yield};

use crate::{sntp, state, udp_client};

const DEFAULT_NTP_SERVER: [u8; 4] = [162, 159, 200, 1];
const NTP_PORT: u16 = 123;
const EXCHANGE_DEADLINE_MS: i64 = 3000;
const RESEND_MS: i64 = 400;

pub fn sync_once() -> Option<i64> {
    let req = sntp::build_request();
    let t0 = mk_time_millis();
    let mut last_send = t0 - RESEND_MS;
    while mk_time_millis().wrapping_sub(t0) <= EXCHANGE_DEADLINE_MS {
        let due = mk_time_millis().wrapping_sub(last_send) >= RESEND_MS;
        let sent = due
            && udp_client::send_to(
                state::udp_port(), state::LOCAL_PORT, DEFAULT_NTP_SERVER, NTP_PORT, &req,
            )
            .is_ok();
        if sent {
            last_send = mk_time_millis();
        }
        match udp_client::recv_from(state::udp_port(), state::LOCAL_PORT) {
            Ok(dg) if dg.src == DEFAULT_NTP_SERVER && dg.src_port == NTP_PORT => {
                return apply(&dg.payload);
            }
            _ => {
                mk_yield();
            }
        }
    }
    None
}

fn apply(payload: &[u8]) -> Option<i64> {
    let correct_ms = sntp::parse_reply(payload).ok()?;
    if mk_time_adjust(correct_ms) == 0 {
        Some(correct_ms as i64)
    } else {
        None
    }
}
