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

//! The Wi-Fi control protocol the Settings panel speaks, distinct from net_core's
//! link protocol and tagged with its own magic so the two never collide. Status
//! and scan are answered instantly; connect runs the whole join.

use crate::status;

use super::connect::{connect, disconnect, ConnectResult, Session};
use super::radio::Radio;
use super::scanner::Scanner;
use super::Stage;

/// The control family tag, distinct from net_core's `MAGIC_NNET`.
const WIFI_MAGIC: u32 = 0x5749_4649;
/// Control header: magic, u16 op, u32 request id.
const WIFI_HDR: usize = 10;
/// Join a network and associate.
const OP_CONNECT: u16 = 1;
/// Tear the session down and clear the keys.
const OP_DISCONNECT: u16 = 2;
/// Return the networks the background scanner has collected.
const OP_SCAN: u16 = 3;
/// Report how far bring-up got, so the panel can show it on a serial-less machine.
const OP_STATUS: u16 = 4;

/// Handle one control request. Returns the reply length, or `None` if it is not a
/// well-formed control frame (so net_core traffic is never misread as control).
pub(super) fn control(
    req: &[u8],
    radio: &mut Radio,
    session: &mut Option<Session>,
    scanner: &Scanner,
    stage: Stage,
    out: &mut [u8],
) -> Option<usize> {
    if req.len() < WIFI_HDR || u32::from_le_bytes([req[0], req[1], req[2], req[3]]) != WIFI_MAGIC {
        return None;
    }
    let op = u16::from_le_bytes([req[4], req[5]]);
    let rid = u32::from_le_bytes([req[6], req[7], req[8], req[9]]);
    let body = &req[WIFI_HDR..];
    out[0..4].copy_from_slice(&WIFI_MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&op.to_le_bytes());
    out[6..10].copy_from_slice(&rid.to_le_bytes());

    match op {
        OP_STATUS => status_reply(stage, out),
        OP_SCAN => scan_reply(radio, scanner, out),
        OP_CONNECT => connect_reply(body, radio, session, out),
        OP_DISCONNECT => disconnect_reply(radio, session, out),
        _ => code_reply(-1, out),
    }
}

// The bring-up stage in one byte, so the panel can show how far the radio got even
// when it never came up.
fn status_reply(stage: Stage, out: &mut [u8]) -> Option<usize> {
    out[WIFI_HDR] = stage as u8;
    Some(WIFI_HDR + 1)
}

// The scan is answered instantly from the background scanner's running picture.
// The reply carries three little-endian counters before the network list: passes
// taken, raw frames the ring delivered, and beacons parsed. Zero passes means the
// scanner never got the CPU; passes but zero raw means the ring delivered nothing;
// raw but zero beacons means frames arrive but do not parse.
fn scan_reply(radio: &Radio, scanner: &Scanner, out: &mut [u8]) -> Option<usize> {
    let n = match radio {
        Radio::Up(_) => {
            out[WIFI_HDR..WIFI_HDR + 4].copy_from_slice(&scanner.steps.to_le_bytes());
            out[WIFI_HDR + 4..WIFI_HDR + 8].copy_from_slice(&scanner.raw.to_le_bytes());
            out[WIFI_HDR + 8..WIFI_HDR + 12].copy_from_slice(&scanner.beacons.to_le_bytes());
            log_scan(scanner);
            12 + scanner.cache().encode(&mut out[WIFI_HDR + 12..])
        }
        Radio::Down => {
            out[WIFI_HDR..WIFI_HDR + 12].fill(0);
            out[WIFI_HDR + 12] = 0;
            13
        }
    };
    Some(WIFI_HDR + n)
}

// A connect carries its handshake progress back so the panel can show where a
// failed join stopped:
// [status i32][sent u32][recv u32][data u32][eapol u32][state u8].
fn connect_reply(
    body: &[u8],
    radio: &mut Radio,
    session: &mut Option<Session>,
    out: &mut [u8],
) -> Option<usize> {
    let r = match radio {
        Radio::Up(up) => connect(body, &mut up.link, &mut up.keys, &up.regs, session),
        Radio::Down => ConnectResult {
            code: -1,
            sent: 0,
            recv: 0,
            data: 0,
            eapol: 0,
            probe: 0,
            deauth: 0,
            to_us: 0,
            state: 0,
        },
    };
    out[10..14].copy_from_slice(&r.code.to_le_bytes());
    out[14..18].copy_from_slice(&r.sent.to_le_bytes());
    out[18..22].copy_from_slice(&r.recv.to_le_bytes());
    out[22..26].copy_from_slice(&r.data.to_le_bytes());
    out[26..30].copy_from_slice(&r.eapol.to_le_bytes());
    out[30..34].copy_from_slice(&r.probe.to_le_bytes());
    out[34..38].copy_from_slice(&r.deauth.to_le_bytes());
    out[38..42].copy_from_slice(&r.to_us.to_le_bytes());
    out[42] = r.state;
    Some(43)
}

fn disconnect_reply(
    radio: &mut Radio,
    session: &mut Option<Session>,
    out: &mut [u8],
) -> Option<usize> {
    let status = match radio {
        Radio::Up(up) => disconnect(&mut up.link, &mut up.keys, session),
        Radio::Down => -1,
    };
    code_reply(status, out)
}

fn code_reply(status: i32, out: &mut [u8]) -> Option<usize> {
    out[10..14].copy_from_slice(&status.to_le_bytes());
    Some(14)
}

// The scan pipeline counters as a debug line, silenced by default (the panel
// shows the same numbers). Left in the code for the next hardware bring-up.
fn log_scan(scanner: &Scanner) {
    status::debug(b"[rtl8821ce] scan steps=");
    status::debug_number(scanner.steps);
    status::debug(b" rx=");
    status::debug_number(scanner.raw);
    status::debug(b" beacons=");
    status::debug_number(scanner.beacons);
    status::debug(b" networks=");
    status::debug_number(scanner.cache().count() as u32);
    status::debug(b"\n");
}
