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

use super::control::note_control;
use super::route_reply::route_reply;
use crate::gateway_client::{self, is_pushed_message, parse_blob, E_RECV_TIMEOUT};
use crate::protocol::WIRE_PACKET_MAX;
use crate::setup;
use crate::state::TABLE;
use crate::trace;

/// Frames to take in one pass.
///
/// A gateway holds messages for a client that was not listening and releases
/// them together, so stopping after one would leave the rest waiting for the
/// next caller. The cap is what keeps a talkative gateway from holding the
/// server inside a single drain.
const BURST: usize = 16;

/// Take whatever the gateway has pushed and route it.
///
/// A gateway forwards messages addressed to us as they arrive, so this is the
/// only place a reply can enter. The frame is authenticated under the session
/// key before its kind is believed: the kind sits outside the sealed part and
/// anyone on the path can set it.
///
/// `wait_ms` is how long an empty link is given to produce something. A client
/// waiting on a reply can afford to wait; the idle pump cannot, because time
/// spent here is time the capsule is not answering anyone.
pub fn drain_stream(wait_ms: i64) {
    let tcp_port = setup::tcp_port();
    let gateway = match TABLE.lock().gateway() {
        Some(gateway) if tcp_port != 0 => gateway,
        _ => return,
    };
    let mut chunk = vec![0u8; WIRE_PACKET_MAX];
    for pass in 0..BURST {
        // Only the first pass waits. Once one frame is in hand the rest of the
        // burst is whatever is already buffered, and pausing for more would
        // charge the full budget to every message in it.
        let budget = if pass == 0 { wait_ms } else { 0 };
        let frame = match gateway_client::recv(tcp_port, gateway, &mut chunk, budget) {
            Ok(frame) => frame,
            Err(e) => {
                // An empty wait is the normal state of a link with nothing in
                // flight. Anything else is the link itself in trouble.
                if e != E_RECV_TIMEOUT {
                    trace::say_num(b"gateway link error", e as u64);
                }
                return;
            }
        };
        if frame.len == 0 {
            return;
        }
        // A text frame is a control message in the clear. It is never a
        // pushed mix message, so it does not go near the session key.
        if frame.text {
            if note_control(&chunk[..frame.len]) {
                // Allowance is granted per session and spent per packet, so
                // running out is a state to leave rather than a failure to
                // report. Asking again is what the gateway expects; without
                // it every later packet is priced, refused, and dropped.
                let _ = gateway_client::claim_free_bandwidth(tcp_port, gateway.stream);
                trace::say(b"asked the gateway for allowance again");
            }
            continue;
        }
        accept(tcp_port, &chunk[..frame.len], &gateway.shared_key);
    }
}

fn accept(tcp_port: u32, frame: &[u8], key: &[u8; 32]) {
    trace::say_num(b"gateway frame bytes", frame.len() as u64);
    let Some(incoming) = parse_blob(frame, key) else {
        trace::say(b"frame dropped: failed to authenticate under the session key");
        return;
    };
    if !is_pushed_message(incoming.kind) {
        trace::say_num(b"frame ignored: kind", incoming.kind as u64);
        return;
    }
    route_reply(tcp_port, &incoming.plaintext);
}
