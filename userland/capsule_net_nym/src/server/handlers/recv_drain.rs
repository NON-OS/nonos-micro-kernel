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

use alloc::vec;

use super::route_reply::route_reply;
use crate::gateway_client::{self, is_pushed_message, parse_blob};
use crate::protocol::WIRE_PACKET_MAX;
use crate::setup;
use crate::state::TABLE;
use crate::trace;

/// Take whatever the gateway has pushed and route it.
///
/// A gateway forwards messages addressed to us as they arrive, so this is the
/// only place a reply can enter. The frame is authenticated under the session
/// key before its kind is believed: the kind sits outside the sealed part and
/// anyone on the path can set it.
pub fn drain_stream() {
    let tcp_port = setup::tcp_port();
    let gateway = match TABLE.lock().gateway() {
        Some(gateway) if tcp_port != 0 => gateway,
        _ => return,
    };
    let mut chunk = vec![0u8; WIRE_PACKET_MAX];
    let Ok(n) = gateway_client::recv(tcp_port, gateway, &mut chunk) else {
        return;
    };
    if n == 0 {
        return;
    }
    trace::say_num(b"gateway frame bytes", n as u64);
    let Some(incoming) = parse_blob(&chunk[..n], &gateway.shared_key) else {
        trace::say(b"frame dropped: failed to authenticate under the session key");
        return;
    };
    if !is_pushed_message(incoming.kind) {
        trace::say_num(b"frame ignored: kind", incoming.kind as u64);
        return;
    }
    route_reply(&incoming.plaintext);
}
