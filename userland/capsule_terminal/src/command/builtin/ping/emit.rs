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
