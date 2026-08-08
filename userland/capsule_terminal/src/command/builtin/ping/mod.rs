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

//! Network `ping`: one-shot ICMP echo to a host through the net.ip service.

mod emit;
mod icmp;
mod job;
mod lookup;
mod parse;
mod poll;
mod probe;
mod resolve;
mod send;

pub use emit::emit_probe;
pub use job::PingJob;

use crate::command::output::Output;

const IP_SERVICE: &[u8] = b"net.ip";
const IP_MAGIC: u32 = 0x4E49_5034;
const HDR_LEN: usize = 20;
const OP_SEND_PACKET: u16 = 4;
const OP_POLL_PACKET: u16 = 5;
const PROTO_ICMP: u8 = 1;
const PING_ID: u16 = 0x4E4F;
const DEADLINE_MS: i64 = 1000;
// Echo requests the synchronous `ping` sends, one per sequence number, so a
// single dropped packet no longer reads as a dead host.
const PING_COUNT: u16 = 4;

pub fn run(out: &mut Output<'_>, argv: &[&[u8]]) {
    if argv.len() < 2 {
        out.writeln(b"usage: ping <host>");
        return;
    }
    let dst = match resolve::resolve(argv[1]) {
        resolve::Resolved::Ip(ip) => ip,
        resolve::Resolved::NoService => return out.writeln(b"ping: dns service unavailable"),
        resolve::Resolved::Timeout => {
            return out.writeln(b"ping: dns query timed out (no reply in 6s)")
        }
        resolve::Resolved::ServFail => return out.writeln(b"ping: dns lookup failed (servfail)"),
        resolve::Resolved::Unknown => return out.writeln(b"ping: unknown host"),
    };
    let Some(port) = probe::lookup_ip_service() else {
        out.writeln(b"ping: net unavailable");
        return;
    };
    emit::emit_target(out, argv[1], &dst);

    let mut stats = emit::Stats::new();
    for seq in 1..=PING_COUNT {
        match probe::probe(port, dst, seq) {
            probe::Probe::Reply(rtt) => {
                stats.record(rtt);
                emit::emit_reply_seq(out, &dst, seq, rtt);
            }
            probe::Probe::Timeout | probe::Probe::Unreachable => {
                stats.miss();
                emit::emit_timeout_seq(out, seq);
            }
            // A routing or configuration fault will not fix itself between
            // packets, so report it once and stop rather than repeat it four
            // times.
            fatal => {
                emit::emit_fatal(out, fatal);
                break;
            }
        }
    }
    emit::emit_summary(out, argv[1], &stats);
}

// Job-submission variant of `run`: same host resolution and target
// announcement, but hands the echo-request/echo-reply poll off as a
// `PingJob` instead of looping it to completion inline.
pub fn prepare(out: &mut Output<'_>, argv: &[&[u8]]) -> Option<PingJob> {
    if argv.len() < 2 {
        out.writeln(b"usage: ping <host>");
        return None;
    }
    let dst = match resolve::resolve(argv[1]) {
        resolve::Resolved::Ip(ip) => ip,
        resolve::Resolved::NoService => {
            out.writeln(b"ping: dns service unavailable");
            return None;
        }
        resolve::Resolved::Timeout => {
            out.writeln(b"ping: dns query timed out (no reply in 6s)");
            return None;
        }
        resolve::Resolved::ServFail => {
            out.writeln(b"ping: dns lookup failed (servfail)");
            return None;
        }
        resolve::Resolved::Unknown => {
            out.writeln(b"ping: unknown host");
            return None;
        }
    };
    let Some(port) = probe::lookup_ip_service() else {
        out.writeln(b"ping: net unavailable");
        return None;
    };
    emit::emit_target(out, argv[1], &dst);
    Some(PingJob::new(port, dst, 1))
}
