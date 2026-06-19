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
mod lookup;
mod parse;
mod poll;
mod probe;
mod resolve;
mod send;

use crate::command::output::Output;

const IP_SERVICE: &[u8] = b"net.ip";
const IP_MAGIC: u32 = 0x4E49_5034;
const HDR_LEN: usize = 20;
const OP_SEND_PACKET: u16 = 4;
const OP_POLL_PACKET: u16 = 5;
const PROTO_ICMP: u8 = 1;
const PING_ID: u16 = 0x4E4F;
const PING_SEQ: u16 = 1;
const DEADLINE_MS: i64 = 1000;

pub fn run(out: &mut Output<'_>, argv: &[&[u8]]) {
    if argv.len() < 2 {
        out.writeln(b"usage: ping <host>");
        return;
    }
    let dst = match resolve::resolve(argv[1]) {
        resolve::Resolved::Ip(ip) => ip,
        resolve::Resolved::NoService => return out.writeln(b"ping: dns service unavailable"),
        resolve::Resolved::Timeout => return out.writeln(b"ping: dns query timed out (no reply in 6s)"),
        resolve::Resolved::ServFail => return out.writeln(b"ping: dns lookup failed (servfail)"),
        resolve::Resolved::Unknown => return out.writeln(b"ping: unknown host"),
    };
    let Some(port) = probe::lookup_ip_service() else {
        out.writeln(b"ping: net unavailable");
        return;
    };
    emit::emit_target(out, argv[1], &dst);
    match probe::probe(port, dst) {
        probe::Probe::Reply(rtt) => emit::emit_reply(out, &dst, rtt),
        probe::Probe::NoRoute => out.writeln(b"no route to host"),
        probe::Probe::NotReady => out.writeln(b"ping: network not ready"),
        probe::Probe::Unreachable => out.writeln(b"ping: destination unreachable (no ARP reply)"),
        probe::Probe::Timeout => out.writeln(b"request timed out"),
        probe::Probe::SendFailed => out.writeln(b"ping: send failed"),
    }
}
