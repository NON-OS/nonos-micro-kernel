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

use core::sync::atomic::{AtomicI64, Ordering};
use nonos_libc::mk_uptime_ms;

use super::connect_tick::{connected, gateway_lost};
use crate::setup;
use crate::state::TABLE;

/// How often the link is pinged. Well inside the minute most gateways allow
/// an idle connection, and rare enough that it costs nothing.
const PING_EVERY_MS: i64 = 20_000;

static LAST_PING_MS: AtomicI64 = AtomicI64::new(0);

/// Keep the gateway link open between packets.
///
/// A session carries nothing while a client sits idle, and a gateway closes
/// a connection it sees no traffic on. The client then discovers this by
/// having a packet refused, which costs whoever sent it that request. A ping
/// keeps the link alive so the first packet after a quiet spell still lands.
pub fn keepalive_tick() {
    if !connected() {
        return;
    }
    let now = mk_uptime_ms();
    if now - LAST_PING_MS.load(Ordering::Relaxed) < PING_EVERY_MS {
        return;
    }
    LAST_PING_MS.store(now, Ordering::Relaxed);

    let Some(gateway) = TABLE.lock().gateway() else {
        return;
    };
    if crate::gateway_client::ping(setup::tcp_port(), gateway).is_err() {
        // Already gone. Drop the binding now rather than letting the next
        // packet be the thing that discovers it.
        gateway_lost();
    }
}
