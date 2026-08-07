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

use crate::command::output::Output;
use crate::term::util::{copy_into, format_u64};

use super::probe::Probe;

pub fn emit_target(out: &mut Output<'_>, host: &[u8], ip: &[u8; 4]) {
    let mut line = [0u8; 128];
    let mut k = 0;
    k += copy_into(&mut line[k..], b"PING ");
    k += copy_into(&mut line[k..], host);
    k += copy_into(&mut line[k..], b" (");
    k += fmt_ipv4(&mut line[k..], ip);
    k += copy_into(&mut line[k..], b")");
    out.writeln(&line[..k]);
    // ICMP is not a stream, so a SOCKS5 proxy cannot carry it and this leaves
    // directly however the rest of the machine is routed. Someone who assumes
    // everything is anonymised is exactly who needs telling.
    if crate::mixnet::routed() {
        out.writeln(b"note: icmp cannot cross the mixnet, this leaves directly");
    }
}

pub fn emit_reply(out: &mut Output<'_>, dst: &[u8; 4], rtt: u64) {
    let mut line = [0u8; 64];
    let mut k = 0;
    k += copy_into(&mut line[k..], b"reply from ");
    k += fmt_ipv4(&mut line[k..], dst);
    k += copy_into(&mut line[k..], b": time=");
    k += format_u64(rtt, &mut line[k..]);
    k += copy_into(&mut line[k..], b" ms");
    out.writeln(&line[..k]);
}

fn fmt_ipv4(out: &mut [u8], ip: &[u8; 4]) -> usize {
    let mut k = 0;
    for (i, &b) in ip.iter().enumerate() {
        if i > 0 {
            out[k] = b'.';
            k += 1;
        }
        k += format_u64(b as u64, &mut out[k..]);
    }
    k
}

// Shared terminal outcome for a finished probe: writes the same line the
// synchronous `ping` builtin always has, and returns an exit status.
pub fn emit_probe(out: &mut Output<'_>, dst: [u8; 4], probe: Probe) -> i32 {
    match probe {
        Probe::Reply(rtt) => {
            emit_reply(out, &dst, rtt);
            0
        }
        Probe::NoRoute => {
            out.writeln(b"no route to host");
            1
        }
        Probe::NotReady => {
            out.writeln(b"ping: network not ready");
            1
        }
        Probe::Unreachable => {
            out.writeln(b"ping: destination unreachable (no ARP reply)");
            1
        }
        Probe::Timeout => {
            out.writeln(b"request timed out");
            1
        }
        Probe::SendFailed => {
            out.writeln(b"ping: send failed");
            1
        }
    }
}

// Running totals for a multi-packet ping, so the summary can report loss and
// round-trip spread the way people expect from ping.
pub struct Stats {
    sent: u32,
    recv: u32,
    min: u64,
    max: u64,
    sum: u64,
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

impl Stats {
    pub fn new() -> Self {
        Self { sent: 0, recv: 0, min: u64::MAX, max: 0, sum: 0 }
    }

    pub fn record(&mut self, rtt: u64) {
        self.sent += 1;
        self.recv += 1;
        self.sum += rtt;
        self.min = self.min.min(rtt);
        self.max = self.max.max(rtt);
    }

    pub fn miss(&mut self) {
        self.sent += 1;
    }
}

// One reply line: "reply from 10.0.2.2: seq=1 time=3 ms".
pub fn emit_reply_seq(out: &mut Output<'_>, dst: &[u8; 4], seq: u16, rtt: u64) {
    let mut line = [0u8; 80];
    let mut k = 0;
    k += copy_into(&mut line[k..], b"reply from ");
    k += fmt_ipv4(&mut line[k..], dst);
    k += copy_into(&mut line[k..], b": seq=");
    k += format_u64(seq as u64, &mut line[k..]);
    k += copy_into(&mut line[k..], b" time=");
    k += format_u64(rtt, &mut line[k..]);
    k += copy_into(&mut line[k..], b" ms");
    out.writeln(&line[..k]);
}

// One dropped-packet line: "request timed out: seq=2".
pub fn emit_timeout_seq(out: &mut Output<'_>, seq: u16) {
    let mut line = [0u8; 48];
    let mut k = 0;
    k += copy_into(&mut line[k..], b"request timed out: seq=");
    k += format_u64(seq as u64, &mut line[k..]);
    out.writeln(&line[..k]);
}

// A fault that will not clear between packets: report it once, then stop.
pub fn emit_fatal(out: &mut Output<'_>, probe: Probe) {
    match probe {
        Probe::NoRoute => out.writeln(b"no route to host"),
        Probe::NotReady => out.writeln(b"ping: network not ready"),
        _ => out.writeln(b"ping: send failed"),
    }
}

// The closing summary: transmitted/received/loss, and the round-trip spread when
// at least one reply came back.
pub fn emit_summary(out: &mut Output<'_>, host: &[u8], stats: &Stats) {
    let mut head = [0u8; 96];
    let mut k = 0;
    k += copy_into(&mut head[k..], b"--- ");
    k += copy_into(&mut head[k..], host);
    k += copy_into(&mut head[k..], b" ping statistics ---");
    out.writeln(&head[..k]);

    let mut counts = [0u8; 80];
    let mut k = 0;
    k += format_u64(stats.sent as u64, &mut counts[k..]);
    k += copy_into(&mut counts[k..], b" transmitted, ");
    k += format_u64(stats.recv as u64, &mut counts[k..]);
    k += copy_into(&mut counts[k..], b" received, ");
    let loss =
        if stats.sent > 0 { (stats.sent - stats.recv) as u64 * 100 / stats.sent as u64 } else { 0 };
    k += format_u64(loss, &mut counts[k..]);
    k += copy_into(&mut counts[k..], b"% loss");
    out.writeln(&counts[..k]);

    if stats.recv > 0 {
        let avg = stats.sum / stats.recv as u64;
        let mut rtt = [0u8; 80];
        let mut k = 0;
        k += copy_into(&mut rtt[k..], b"rtt min/avg/max = ");
        k += format_u64(stats.min, &mut rtt[k..]);
        rtt[k] = b'/';
        k += 1;
        k += format_u64(avg, &mut rtt[k..]);
        rtt[k] = b'/';
        k += 1;
        k += format_u64(stats.max, &mut rtt[k..]);
        k += copy_into(&mut rtt[k..], b" ms");
        out.writeln(&rtt[..k]);
    }
}
